use std::collections::HashMap;

use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::{
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    time::{Duration, OffsetDateTime},
    Request,
};

use crate::auth::User;

use super::{Claims, JwtConfig};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct JwtToken(String);

pub struct JwtTokenProvider<'r> {
    key: &'r EncodingKey,
}

impl<'r> JwtTokenProvider<'r> {
    pub fn create_token(&self, user: &User) -> Result<JwtToken, JwtTokenError> {
        let now = OffsetDateTime::now_utc();

        let mut other = HashMap::new();
        other.insert("roles".to_string(), user.roles().iter().collect());
        
        let claims = Claims {
            sub: user.username().to_string(),
            nbf: now.into(),
            iat: now.into(),
            exp: (now + Duration::days(180)).into(),
            other,
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

#[derive(Debug, thiserror::Error)]
pub enum JwtTokenError {
    #[error("Failed to create token: {0}")]
    CreateToken(#[from] jsonwebtoken::errors::Error),
}