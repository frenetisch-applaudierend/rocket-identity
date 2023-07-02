use super::{User, UserStore};
use crate::{
    auth::{self, hasher::PasswordHasher},
    util::Result,
};

pub struct InMemoryRepository {
    users: Vec<User>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn add_user(&mut self, username: &str, password: &str, hasher: &dyn PasswordHasher) {
        let id = (self.users.len() + 1).to_string();
        let auth_user = auth::User {
            id: id.clone(),
            username: username.to_string(),
        };

        let password_hash = hasher
            .hash_password(&auth_user, password)
            .expect("Could not hash password");

        let repo_user = User {
            id,
            username: username.to_string(),
            password_hash: Some(password_hash),
        };

        self.users.push(repo_user);
    }
}

#[rocket::async_trait]
impl UserStore for InMemoryRepository {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        for user in &self.users {
            if user.username == username {
                return Ok(Some(user.clone()));
            }
        }

        Ok(None)
    }
}

impl Default for InMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}
