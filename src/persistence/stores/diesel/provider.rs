use diesel::Connection;
use rocket::{Orbit, Request, Rocket};

#[rocket::async_trait]
pub trait DieselConnectionProvider: Sized + Send + Sync + 'static {
    type Conn: Connection;

    async fn create_from_request(req: &Request<'_>) -> Result<Self, ProviderCreationError>;

    async fn create_from_rocket(rocket: &Rocket<Orbit>) -> Result<Self, ProviderCreationError>;

    async fn with_connection<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Self::Conn) -> R + Send + 'static,
        R: Send + 'static;
}

#[derive(Debug, thiserror::Error)]
#[error("Could not create database connection provider")]
pub struct ProviderCreationError;
