use std::error::Error;

use crate::{hashers::PasswordHash, User};

/// Trait for an object that persists users.
#[rocket::async_trait]
pub trait UserStoreScope: Send + Sync + 'static {
    /// Find a user by their username.
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, FindUserError>;

    /// Add a user to the store.
    async fn add_user(
        &mut self,
        user: &User,
        password_hash: Option<&PasswordHash>,
    ) -> Result<(), AddUserError>;

    /// Retrieve the password hash for a given user.
    async fn password_hash(&self, user: &User) -> Result<Option<PasswordHash>, PasswordHashError>;

    /// Set the password hash for a given user.
    async fn set_password_hash(
        &mut self,
        user: &User,
        password_hash: &PasswordHash,
    ) -> Result<(), PasswordHashError>;
}

#[derive(Debug, thiserror::Error)]
pub enum FindUserError {
    #[error("an error occurred")]
    Other(#[from] Box<dyn Error>),
}

#[derive(Debug, thiserror::Error)]
pub enum AddUserError {
    #[error("user already exists")]
    UsernameExists,

    #[error("an error occurred while trying to add a user")]
    Other(#[from] Box<dyn Error>),
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordHashError {
    #[error("user was not found")]
    UserNotFound,

    #[error("an error occurred while trying to hash the password")]
    Other(#[from] Box<dyn Error>),
}