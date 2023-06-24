use std::error::Error;

use crate::auth::User;

#[rocket::async_trait]
pub trait AuthenticationScheme: Send + Sync {
    async fn autenticate(&self, req: &rocket::Request) -> Outcome;

    fn challenge(&self) -> String;
}

pub type Outcome = rocket::outcome::Outcome<User, (rocket::http::Status, Box<dyn Error>), ()>;

pub(crate) struct AuthenticationSchemes(pub Vec<Box<dyn AuthenticationScheme>>);
