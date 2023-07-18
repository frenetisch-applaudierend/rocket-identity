use std::collections::HashMap;

use crate::stores::impls::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct InMemoryUserStore {
    users: HashMap<String, UserEntry>,
}

#[derive(Debug, Clone)]
struct UserEntry {
    user: User,
    password_hash: Option<PasswordHash>,
}

impl InMemoryUserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

#[rocket::async_trait]
impl UserStore for InMemoryUserStore {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        Ok(self.users.get(username).map(|e| e.user.clone()))
    }

    async fn add_user(&mut self, user: &User, password_hash: Option<&PasswordHash>) -> Result<()> {
        self.users.insert(
            user.username.to_string(),
            UserEntry {
                user: user.clone(),
                password_hash: password_hash.map(|h| h.clone()),
            },
        );

        Ok(())
    }

    async fn password_hash(&self, user: &User) -> Result<Option<PasswordHash>> {
        let Some(entry) = self.users.get(&user.username) else {
            return Err(Box::new(InMemoryStoreError::UserNotFound));
        };

        Ok(entry.password_hash.clone())
    }

    async fn set_password_hash(&mut self, user: &User, password_hash: &PasswordHash) -> Result<()> {
        let Some(entry) = self.users.get_mut(&user.username) else {
            return Err(Box::new(InMemoryStoreError::UserNotFound));
        };

        entry.password_hash = Some(password_hash.clone());

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
enum InMemoryStoreError {
    #[error("User not found")]
    UserNotFound,
}
