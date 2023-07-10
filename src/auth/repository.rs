use tokio::sync::RwLock;

use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

use crate::{
    auth,
    persistence::{self, UserStore},
    util::DynError,
};

use super::{hasher::PasswordHasher, scheme::AuthenticationError, Claims, Roles, User, UserData};

pub struct UserRepository {
    pub user_store: RwLock<Box<dyn UserStore>>,
    pub password_hasher: Box<dyn PasswordHasher>,
}

impl UserRepository {
    pub fn new(
        user_store: impl UserStore + 'static,
        password_hasher: impl PasswordHasher + 'static,
    ) -> Self {
        Self {
            user_store: RwLock::new(Box::new(user_store)),
            password_hasher: Box::new(password_hasher),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<User, LoginError> {
        let user_store = self.user_store.read().await;

        let Some(repo_user) = user_store.find_user_by_username(username).await.map_err(|err| LoginError::Other(err))? else {
            return Err(LoginError::UserNotFound);
        };

        let user_data = UserData {
            id: repo_user.id,
            username: repo_user.username,
            claims: Claims::from_inner(repo_user.claims),
            roles: Roles::from_inner(repo_user.roles),
        };

        let Some(password_hash) = repo_user.password_hash else {
            return Err(LoginError::MissingPassword);
        };

        if !self
            .password_hasher
            .verify_password(&user_data, &password_hash, password)
            .map_err(LoginError::Other)?
        {
            return Err(LoginError::IncorrectPassword);
        }

        Ok(auth::User::from_data(user_data))
    }

    pub async fn add_user(
        &self,
        data: UserData,
        password: Option<&str>,
    ) -> Result<User, AddUserError> {
        let password_hash = password
            .map(|p| {
                self.password_hasher
                    .hash_password(&data, p)
                    .map_err(AddUserError::Other)
            })
            .transpose()?;

        let user = persistence::User::from_data(data, password_hash);

        let mut user_store = self.user_store.write().await;

        let id = user_store
            .add_user(&user)
            .await
            .map_err(AddUserError::Other)?;

        Ok(auth::User::from_data(UserData {
            id: Some(id),
            username: user.username,
            claims: Claims::from_inner(user.claims),
            roles: Roles::from_inner(user.roles),
        }))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r UserRepository {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(req.user_repository())
    }
}

pub trait UserRepositoryRequestExt {
    fn user_repository(&self) -> &UserRepository;
}

impl<'r> UserRepositoryRequestExt for Request<'r> {
    fn user_repository(&self) -> &UserRepository {
        self.rocket()
            .state::<UserRepository>()
            .expect("Missing required UserRepository")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("The user could not be found")]
    UserNotFound,

    #[error("The user could be found but has no password")]
    MissingPassword,

    #[error("The provided password does not match the users")]
    IncorrectPassword,

    #[error("Could not acquire read lock on user store")]
    Lock,

    #[error("Some other error happened")]
    Other(DynError),
}

impl From<LoginError> for AuthenticationError {
    fn from(err: LoginError) -> Self {
        match err {
            LoginError::UserNotFound => AuthenticationError::Unauthenticated,
            LoginError::MissingPassword => AuthenticationError::Unauthenticated,
            LoginError::IncorrectPassword => AuthenticationError::Unauthenticated,
            LoginError::Lock => AuthenticationError::Other(None),
            LoginError::Other(err) => AuthenticationError::Other(Some(err)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AddUserError {
    #[error("Could not acquire write lock on user store")]
    Lock,

    #[error("Some other error happened")]
    Other(DynError),
}
