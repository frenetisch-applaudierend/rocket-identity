use rocket::{request::{FromRequest, Outcome}, Request};

use crate::{persistence::UserStore, util::DynError};

use super::{hasher::PasswordHasher, scheme::AuthenticationError, Claims, Roles, User, UserData};

#[rocket::async_trait]
pub trait UserRepository: Sync + Send {
    async fn login(&self, username: &str, password: &str) -> Result<User, LoginError>;

    fn user_from_data(&self, user_data: UserData) -> User;
}

pub(crate) struct DefaultUserRepository<S, H>
where
    S: UserStore,
    H: PasswordHasher,
{
    pub user_store: S,
    pub password_hasher: H,
}

#[rocket::async_trait]
impl<S, H> UserRepository for DefaultUserRepository<S, H>
where
    S: UserStore,
    H: PasswordHasher,
{
    async fn login(&self, username: &str, password: &str) -> Result<User, LoginError> {
        let Some(repo_user) = self.user_store.find_user_by_username(username).await.map_err(|err| LoginError::Other(err))? else {
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

        Ok(self.user_from_data(user_data))
    }

    fn user_from_data(&self, user_data: UserData) -> User {
        User::from_data(user_data)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r dyn UserRepository {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(req.user_repository())
    }
}

pub trait UserRepositoryRequestExt {
    fn user_repository(&self) -> &dyn UserRepository;
}

impl<'r> UserRepositoryRequestExt for Request<'r> {
    fn user_repository(&self) -> &dyn UserRepository {
        self.rocket()
            .state::<Box<dyn UserRepository>>()
            .expect("Missing required UserRepository")
            .as_ref()
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

    #[error("Some other error happened")]
    Other(DynError),
}

impl From<LoginError> for AuthenticationError {
    fn from(err: LoginError) -> Self {
        match err {
            LoginError::UserNotFound => AuthenticationError::Unauthenticated,
            LoginError::MissingPassword => AuthenticationError::Unauthenticated,
            LoginError::IncorrectPassword => AuthenticationError::Unauthenticated,
            LoginError::Other(err) => AuthenticationError::Other(Some(err)),
        }
    }
}
