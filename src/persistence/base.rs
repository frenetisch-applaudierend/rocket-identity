use super::User;
use crate::util::Result;

#[rocket::async_trait]
pub trait UserStore: Send + Sync {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>>;
}