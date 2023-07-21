use rocket::{Orbit, Request, Rocket};

#[rocket::async_trait]
pub trait DieselConnectionProvider: Sized + Send + Sync + 'static {
    async fn create_from_request(req: &Request<'_>) -> Result<Self, ProviderCreationError>;

    async fn create_from_rocket(rocket: &Rocket<Orbit>) -> Result<Self, ProviderCreationError>;

    async fn with_connection<F, R>(&self, f: F) -> R
    where
        F: FnOnce(DieselConnection) -> R + Send + 'static,
        R: Send + 'static;
}

pub enum DieselConnection<'a> {
    Sqlite(&'a mut diesel::SqliteConnection),
}

#[derive(Debug, thiserror::Error)]
#[error("Could not create database connection provider")]
pub struct ProviderCreationError;
