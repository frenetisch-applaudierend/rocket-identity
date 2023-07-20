use std::{collections::HashMap, sync::Arc};

use rocket::{Orbit, Request, Rocket};
use tokio::sync::RwLock;

use crate::stores::impls::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct InMemoryUserStore {
    users: Arc<RwLock<HashMap<String, UserEntry>>>,
}

#[derive(Debug)]
struct InMemoryUserStoreScope {
    users: Arc<RwLock<HashMap<String, UserEntry>>>,
}

#[derive(Debug, Clone)]
struct UserEntry {
    user: User,
    password_hash: Option<PasswordHash>,
}

impl InMemoryUserStore {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn create_scope(&self) -> InMemoryUserStoreScope {
        InMemoryUserStoreScope {
            users: self.users.clone(),
        }
    }
}

#[rocket::async_trait]
impl UserStore for InMemoryUserStore {
    async fn create_request_scope<'r>(&self, _req: &'r Request<'_>) -> Box<dyn UserStoreScope> {
        Box::new(self.create_scope())
    }

    async fn create_global_scope(
        &self,
        _rocket: &Rocket<Orbit>,
    ) -> Option<Box<dyn UserStoreScope>> {
        Some(Box::new(self.create_scope()))
    }
}

#[rocket::async_trait]
impl UserStoreScope for InMemoryUserStoreScope {
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
            return Err(Box::new(InMemoryStoreError::UserNotFound));
        };

        Ok(entry.password_hash.clone())
    }

    async fn set_password_hash(&mut self, user: &User, password_hash: &PasswordHash) -> Result<()> {
        let mut users = self.users.write().await;

        let Some(entry) = users.get_mut(&user.username) else {
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
