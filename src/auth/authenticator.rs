use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

use super::{LoginError, User};

pub struct Authenticator;

impl Authenticator {
    pub async fn login(&self, username: &str, _password: &str) -> Result<User, LoginError> {
        Ok(User {
            user_name: username.to_string(),
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authenticator {
    type Error = ();

    async fn from_request(_req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(Authenticator)
    }
}
