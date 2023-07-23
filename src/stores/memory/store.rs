use std::{collections::HashMap, sync::Arc};

use rocket::{Orbit, Request, Rocket};
use tokio::sync::RwLock;

use crate::stores::impls::prelude::*;

use super::scope::MemoryStoreScope;

#[derive(Debug, Default, Clone)]
pub struct MemoryStore {
    users: Arc<RwLock<HashMap<String, UserEntry>>>,
}

#[derive(Debug, Clone)]
pub(crate) struct UserEntry {
    pub user: User,
    pub password_hash: Option<PasswordHash>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn create_scope(&self) -> MemoryStoreScope {
        MemoryStoreScope {
            users: self.users.clone(),
        }
    }
}

#[rocket::async_trait]
impl UserStore for MemoryStore {
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
