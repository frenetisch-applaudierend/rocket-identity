use std::sync::Arc;

use rocket::{
    request::{FromRequest, Outcome},
    Request, Sentinel,
};
use tokio::sync::RwLock;

use crate::{
    hashers::PasswordHasher,
    stores::{UserStore, UserStoreScope},
    util::BoxableError,
    Services, User,
};

pub struct UserRepository {
    pub user_store: RwLock<Box<dyn UserStoreScope>>,
    pub password_hasher: Arc<dyn PasswordHasher>,
}

impl UserRepository {
    pub fn new(
        user_store: Box<dyn UserStoreScope>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_store: RwLock::new(user_store),
            password_hasher,
        }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, FindUserError> {
        let user_store = self.user_store.read().await;

        user_store
            .find_user_by_username(username)
            .await
            .map_err(|e| {
                log::error!("Failed to find user by username: {}", e);
                e.boxed().into()
            })
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Result<User, LoginError> {
        let user_store = self.user_store.read().await;

        let user = user_store
            .find_user_by_username(username)
            .await
            .map_err(|e| {
                log::error!("Failed to find user: {}", e);
                LoginError::Other(e.boxed())
            })?;

        let Some(user) = user else {
            return Err(LoginError::UserNotFound);
        };

        let password_hash = user_store.password_hash(&user).await.map_err(|e| {
            log::error!("Failed to retrieve password hash: {}", e);
            LoginError::Other(e.boxed())
        })?;

        let Some(password_hash) = password_hash else {
            return Err(LoginError::MissingPassword);
        };

        if !self
            .password_hasher
            .verify_password(&user, &password_hash, password)
            .map_err(|e| {
                log::error!("Failed to verify password: {}", e);
                LoginError::Other(e)
            })?
        {
            return Err(LoginError::IncorrectPassword);
        }

        Ok(user)
    }

    pub async fn add_user(&self, user: &User, password: Option<&str>) -> Result<(), AddUserError> {
        // Hash the user password
        let password_hash = password
            .map(|p| self.password_hasher.hash_password(user, p))
            .transpose()
            .map_err(|e| {
                log::error!("Failed to hash password: {}", e);
                AddUserError::Other(e)
            })?;

        let mut user_store_guard = self.user_store.write().await;
        let user_store = user_store_guard.as_mut();

        user_store
            .add_user(user, password_hash.as_ref())
            .await
            .map_err(|e| {
                log::error!("Failed to add user: {}", e);
                AddUserError::from(e)
            })?;

        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r UserRepository {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(
            req.local_cache_async(async { req.user_repository().await })
                .await,
        )
    }
}

impl Sentinel for &UserRepository {
    fn abort(rocket: &rocket::Rocket<rocket::Ignite>) -> bool {
        if rocket.state::<Box<dyn UserStore>>().is_none() {
            log::error!("UserStore is not configured but required for UserRepository. Attach Identity::fairing() on your rocket instance and add a UserStore to your configuration.");
            return true;
        }

        if rocket.state::<Arc<dyn PasswordHasher>>().is_none() {
            log::error!("PasswordHasher is not configured but required for UserRepository. Attach Identity::fairing() on your rocket instance and add a PasswordHasher to your configuration.");
            return true;
        }

        false
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FindUserError {
    #[error("Some other error happened")]
    Other(#[from] Box<dyn std::error::Error>),
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("user could not be found")]
    UserNotFound,

    #[error("user has no password")]
    MissingPassword,

    #[error("provided password is incorrect")]
    IncorrectPassword,

    #[error("user could not be authenticated")]
    Other(#[from] Box<dyn std::error::Error>),
}

#[derive(Debug, thiserror::Error)]
pub enum AddUserError {
    #[error("a user with the given username already exists")]
    UsernameExists,

    #[error("user could not be added")]
    Other(#[from] Box<dyn std::error::Error>),
}

impl From<crate::stores::AddUserError> for AddUserError {
    fn from(e: crate::stores::AddUserError) -> Self {
        match e {
            crate::stores::AddUserError::UsernameExists => Self::UsernameExists,
            crate::stores::AddUserError::Other(e) => Self::Other(e),
        }
    }
}
