use std::{sync::Arc, collections::HashMap};

use tokio::sync::RwLock;

use crate::stores::impls::prelude::*;

use super::UserEntry;

#[derive(Debug)]
pub(crate) struct MemoryStoreScope {
    pub users: Arc<RwLock<HashMap<String, UserEntry>>>,
}

#[rocket::async_trait]
impl UserStoreScope for MemoryStoreScope {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let users = self.users.read().await;

        Ok(users.get(username).map(|e| e.user.clone()))
    }

    async fn add_user(&mut self, user: &User, password_hash: Option<&PasswordHash>) -> Result<()> {
        let mut users = self.users.write().await;

        users.insert(
            user.username.to_string(),
            UserEntry {
                user: user.clone(),
                password_hash: password_hash.cloned(),
            },
        );

        Ok(())
    }

    async fn password_hash(&self, user: &User) -> Result<Option<PasswordHash>> {
        let users = self.users.read().await;

        let Some(entry) = users.get(&user.username) else {
            return Err(Box::new(UserNotFound));
        };

        Ok(entry.password_hash.clone())
    }

    async fn set_password_hash(&mut self, user: &User, password_hash: &PasswordHash) -> Result<()> {
        let mut users = self.users.write().await;

        let Some(entry) = users.get_mut(&user.username) else {
            return Err(Box::new(UserNotFound));
        };

        entry.password_hash = Some(password_hash.clone());

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("User not found")]
struct UserNotFound;