use std::error::Error;

use crate::auth::User;

#[rocket::async_trait]
pub trait AuthenticationScheme: Send + Sync {
    fn setup(&self, rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        rocket
    }

    async fn autenticate(&self, req: &rocket::Request) -> Outcome;

    fn challenge(&self) -> String;
}

pub type Outcome = rocket::outcome::Outcome<User, (rocket::http::Status, Box<dyn Error>), ()>;

pub(crate) struct AuthenticationSchemes(Vec<Box<dyn AuthenticationScheme>>);

impl AuthenticationSchemes {
    pub fn from(schemes: Vec<Box<dyn AuthenticationScheme>>) -> Self {
        Self(schemes)
    }

    pub fn iter(&self) -> impl Iterator<Item = &dyn AuthenticationScheme> {
        self.0.iter().map(|b| &**b)
    }
}
