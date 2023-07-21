use rocket::{Request, Rocket, Orbit};

use crate::{util::Result, PasswordHash, User};

#[rocket::async_trait]
pub trait UserStore: Send + Sync + core::fmt::Debug + 'static {
    async fn create_request_scope<'r>(&self, req: &'r Request<'_>) -> Box<dyn UserStoreScope>;

    async fn create_global_scope(&self, rocket: &Rocket<Orbit>) -> Option<Box<dyn UserStoreScope>>;
}

/// Trait for an object that persists users.
#[rocket::async_trait]
pub trait UserStoreScope: Send + Sync + core::fmt::Debug + 'static {
    /// Find a user by their username.
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>>;

    /// Add a user to the store.
    async fn add_user(&mut self, user: &User, password_hash: Option<&PasswordHash>) -> Result<()>;

    /// Retrieve the password hash for a given user.
    async fn password_hash(&self, user: &User) -> Result<Option<PasswordHash>>;

    /// Set the password hash for a given user.
    async fn set_password_hash(&mut self, user: &User, password_hash: &PasswordHash) -> Result<()>;
}
