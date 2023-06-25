use rocket::{
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Request,
};

use crate::auth::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct JwtToken(String);

pub struct JwtTokenProvider;

impl JwtTokenProvider {
    pub fn generate_token(&self, _user: &User) -> JwtToken {
        todo!()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtTokenProvider {
    type Error = ();

    async fn from_request(_req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(JwtTokenProvider)
    }
}
