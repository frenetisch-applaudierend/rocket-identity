use rocket::{Orbit, Request, Rocket};

use super::scope::UserStoreScope;

#[rocket::async_trait]
pub trait UserStore: Send + Sync + core::fmt::Debug + 'static {
    async fn create_request_scope<'r>(&self, req: &'r Request<'_>) -> Box<dyn UserStoreScope>;

    async fn create_global_scope(&self, rocket: &Rocket<Orbit>) -> Option<Box<dyn UserStoreScope>>;
}
