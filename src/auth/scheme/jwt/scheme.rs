use std::collections::HashMap;

use jsonwebtoken::{decode, Algorithm, TokenData, Validation};
use rocket::{serde::json, Request};

use crate::{
    auth::scheme::{AuthenticationError, AuthenticationScheme, Outcome},
    auth::User,
    util::Boxable,
};

use super::JwtConfig;

pub struct JwtBearer {
    challenge: &'static str,
    config: Option<JwtConfig>,
}

type ParsedToken = TokenData<HashMap<String, json::Value>>;

impl JwtBearer {
    pub fn new(config: JwtConfig) -> Self {
        Self {
            challenge: "Bearer",
            config: Some(config),
        }
    }

    async fn authenticate_with_header(header: &str, user: &mut User, req: &Request<'_>) -> Outcome {
        // We expect a Bearer scheme
        let Some(token) = header.strip_prefix("Bearer ") else {
            return Outcome::Forward(());
        };

        let token = token.trim();

        let key = &req
            .rocket()
            .state::<JwtConfig>()
            .expect("Missing JwtConfig")
            .deconding_key;

        let validation = Validation::new(Algorithm::HS256);

        let token = match decode::<HashMap<String, json::Value>>(token, key, &validation) {
            Ok(token) => token,
            Err(err) => {
                return Outcome::Failure(AuthenticationError::InvalidParams(Some(err.boxed())))
            }
        };

        match Self::fill_user_from_token(user, &token) {
            Ok(user) => Outcome::Success(()),
            Err(err) => Outcome::Failure(AuthenticationError::InvalidParams(Some(err.boxed()))),
        }
    }

    fn fill_user_from_token(user: &mut User, token: &ParsedToken) -> Result<(), JwtError> {
        let username = token
            .claims
            .get("sub")
            .ok_or(JwtError::MissingSub)?
            .as_str()
            .ok_or(JwtError::InvalidClaim("sub".to_owned()))?;

        user.id = username.to_owned();
        user.username = username.to_owned();

        Ok(())
    }
}

#[rocket::async_trait]
impl AuthenticationScheme for JwtBearer {
    fn setup(&mut self, rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        rocket.manage(
            self.config
                .take()
                .expect("JwtConfig was already taken or never added"),
        )
    }

    async fn authenticate(&self, user: &mut User, req: &rocket::Request) -> Outcome {
        for header in req.headers().get("Authorization") {
            match (Self::authenticate_with_header(header, user, req)).await {
                Outcome::Success(()) => return Outcome::Success(()),
                Outcome::Failure(err) => return Outcome::Failure(err),
                Outcome::Forward(()) => {}
            }
        }

        // No Authorization headers, we cannot handle the request
        Outcome::Forward(())
    }

    fn challenge_header(&self) -> String {
        self.challenge.to_string()
    }
}

#[derive(Debug, thiserror::Error)]
enum JwtError {
    #[error("Missing sub claim in JWT")]
    MissingSub,

    #[error("Invalid claim value")]
    InvalidClaim(String),
}
