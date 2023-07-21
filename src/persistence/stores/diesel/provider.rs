use diesel::Connection;
use rocket::{Orbit, Request, Rocket};

use crate::util::Result;

#[rocket::async_trait]
pub trait DieselConnectionProvider: Sized + Send + Sync + core::fmt::Debug + 'static {
    type Conn: Connection;

    async fn create_from_request(req: &Request<'_>) -> Result<Self>;

    async fn create_from_rocket(rocket: &Rocket<Orbit>) -> Result<Self>;

    fn with_connection(&self, f: impl FnOnce(&Self::Conn) -> Result<()>) -> Result<()>;
}
