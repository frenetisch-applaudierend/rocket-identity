use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::{
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    time::{Duration, OffsetDateTime},
    Request,
};

use crate::auth::User;
use crate::util::Result;

use super::{Claims, JwtConfig};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct JwtToken(String);

pub struct JwtTokenProvider<'r> {
    key: &'r EncodingKey,
}

impl<'r> JwtTokenProvider<'r> {
    pub fn generate_token(&self, user: &User) -> Result<JwtToken> {
        let now = OffsetDateTime::now_utc();
        let claims = Claims {
            sub: user.username().to_string(),
            nbf: now.into(),
            iat: now.into(),
            exp: (now + Duration::days(180)).into(),
        };

        let token = encode(&Header::default(), &claims, self.key)?;

        Ok(JwtToken(token))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtTokenProvider<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let config = req
            .rocket()
            .state::<JwtConfig>()
            .expect("Missing JwtConfig");

        Outcome::Success(JwtTokenProvider {
            key: &config.encoding_key,
        })
    }
}
