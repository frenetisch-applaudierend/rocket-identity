use crate::{util::Result, User, PasswordHash};

/// Trait for an object that persists users.
#[rocket::async_trait]
pub trait UserStore: Send + Sync {
    /// Find a user by their username.
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>>;

    /// Add a user to the store.
    async fn add_user(&mut self, user: &User, password_hash: Option<&PasswordHash>) -> Result<()>;

    /// Retrieve the password hash for a given user.
    async fn password_hash(&self, user: &User) -> Result<Option<PasswordHash>>;

    /// Set the password hash for a given user.
    async fn set_password_hash(&mut self, user: &User, password_hash: &PasswordHash) -> Result<()>;
}
