use rocket::{
    request::{FromRequest, Outcome},
    Request, Sentinel,
};

use tokio::sync::RwLock;

use crate::{
    auth,
    persistence::{self, UserStore},
};

use super::{hasher::PasswordHasher, Claims, Roles, User, UserData};

pub struct UserRepository<TUserId> {
    pub user_store: RwLock<Box<dyn UserStore<TUserId>>>,
    pub password_hasher: Box<dyn PasswordHasher<TUserId>>,
}

impl<TUserId> UserRepository<TUserId> {
    pub(crate) fn new(
        user_store: Box<dyn UserStore<TUserId>>,
        password_hasher: Box<dyn PasswordHasher<TUserId>>,
    ) -> Self {
        Self {
            user_store: RwLock::new(user_store),
            password_hasher: password_hasher,
        }
    }

    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User<TUserId>, LoginError> {
        let user_store = self.user_store.read().await;

        let Some(repo_user) = user_store.find_user_by_username(username).await.map_err(|err| {
            log::error!("Failed to find user by username: {}", err);
            LoginError::Other
        })? else {
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
            .map_err(|e| {
                log::error!("Failed to verify password: {}", e);
                LoginError::Other
            })?
        {
            return Err(LoginError::IncorrectPassword);
        }

        Ok(auth::User::from_data(user_data))
    }

    pub async fn add_user(
        &self,
        data: UserData<TUserId>,
        password: Option<&str>,
    ) -> Result<User<TUserId>, AddUserError> {
        let password_hash = password
            .map(|p| {
                self.password_hasher.hash_password(&data, p).map_err(|e| {
                    log::error!("Failed to hash password: {}", e);
                    AddUserError::Other
                })
            })
            .transpose()?;

        let mut user = persistence::User::from_data(data, password_hash);

        let mut user_store = self.user_store.write().await;

        // TODO: Check if user already exists

        user_store.add_user(&mut user).await.map_err(|e| {
            log::error!("Failed to add user to store: {}", e);
            AddUserError::Other
        })?;

        Ok(auth::User::from_repo(user))
    }
}

#[rocket::async_trait]
impl<'r, TUserId: 'static> FromRequest<'r> for &'r UserRepository<TUserId> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(req.user_repository())
    }
}

impl<TUserId: 'static> Sentinel for &UserRepository<TUserId> {
    fn abort(rocket: &rocket::Rocket<rocket::Ignite>) -> bool {
        if rocket.state::<UserRepository<TUserId>>().is_none() {
            log::error!("UserRepository is not configured. Attach RocketIdentity::fairing() on your rocket instance.");
            true
        } else {
            false
        }
    }
}

pub trait UserRepositoryAccessor {
    fn user_repository<TUserId: 'static>(&self) -> &UserRepository<TUserId>;
}

impl<'r> UserRepositoryAccessor for Request<'r> {
    fn user_repository<TUserId: 'static>(&self) -> &UserRepository<TUserId> {
        self.rocket()
            .state::<UserRepository<TUserId>>()
            .expect("Missing required UserRepository")
    }
}

impl UserRepositoryAccessor for rocket::Rocket<rocket::Orbit> {
    fn user_repository<TUserId: 'static>(&self) -> &UserRepository<TUserId> {
        self.state().expect("Missing required UserRepository")
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
    Other,
}

#[derive(Debug, thiserror::Error)]
pub enum AddUserError {
    #[error("Some other error happened")]
    Other,
}
