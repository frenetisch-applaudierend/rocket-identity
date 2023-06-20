use std::error::Error;

use crate::auth::User;

#[rocket::async_trait]
pub trait AuthenticationScheme {
    async fn autenticate(&self, req: &rocket::Request) -> Outcome;

    fn challenge(&self) -> String;
}

pub type Outcome = rocket::outcome::Outcome<User, (rocket::http::Status, Box<dyn Error>), ()>;
