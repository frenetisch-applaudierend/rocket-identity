use crate::{auth::UserId, persistence::User, util::Result};

/// Trait for an object that persists users.
#[rocket::async_trait]
pub trait UserStore: Send + Sync {
    /// Find a user by their username.
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>>;

    /// Add a user to the store.
    async fn add_user(&mut self, user: &User) -> Result<UserId>;
}
