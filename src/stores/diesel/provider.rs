use rocket::{Orbit, Request, Rocket};

use crate::stores::UserStoreScope;

#[rocket::async_trait]
pub trait DieselScopeProvider: Send + Sync + 'static {
    type Scope: UserStoreScope;

    async fn create_from_request(req: &Request<'_>) -> Result<Self::Scope, ProviderCreationError>;

    async fn create_from_rocket(
        rocket: &Rocket<Orbit>,
    ) -> Result<Self::Scope, ProviderCreationError>;
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to provide connection")]
pub struct ProviderCreationError;
