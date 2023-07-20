use std::sync::Arc;

use rocket::{
    request::{FromRequest, Outcome},
    Request, Sentinel,
};
use tokio::sync::RwLock;

use crate::{PasswordHasher, Services, User, UserStore, UserStoreScope};

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
                FindUserError::Other
            })
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Result<User, LoginError> {
        let user_store = self.user_store.read().await;

        let Some(user) = user_store.find_user_by_username(username).await
            .map_err(LoginError::to_other("Failed to find user by username"))? else {
            return Err(LoginError::UserNotFound);
        };

        let Some(password_hash) = user_store.password_hash(&user).await
            .map_err(LoginError::to_other("Failed to retrieve password hash"))? else {
            return Err(LoginError::MissingPassword);
        };

        if !self
            .password_hasher
            .verify_password(&user, &password_hash, password)
            .map_err(LoginError::to_other("Failed to verify password"))?
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
            .map_err(AddUserError::to_other("Failed to hash password"))?;

        let mut user_store = self.user_store.write().await;

        // TODO: Check if user already exists

        user_store
            .add_user(user, password_hash.as_ref())
            .await
            .map_err(AddUserError::to_other("Failed to add user to store"))?;

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
    Other,
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

impl LoginError {
    fn to_other(msg: &'static str) -> impl FnOnce(Box<dyn std::error::Error>) -> Self {
        move |e| {
            log::error!("{}: {}", msg, e);
            Self::Other
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AddUserError {
    #[error("Some other error happened")]
    Other,
}

impl AddUserError {
    fn to_other(msg: &'static str) -> impl FnOnce(Box<dyn std::error::Error>) -> Self {
        move |e| {
            log::error!("{}: {}", msg, e);
            Self::Other
        }
    }
}
