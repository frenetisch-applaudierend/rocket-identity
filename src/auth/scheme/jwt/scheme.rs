use std::collections::HashMap;

use jsonwebtoken::{decode, Algorithm, TokenData, Validation};
use rocket::{serde::json, Request};

use crate::{auth::scheme::prelude::*, util::Boxable};

use super::JwtConfig;

pub struct JwtBearer {
    challenge: &'static str,
    config: Option<JwtConfig>,
}

type ParsedToken = TokenData<HashMap<String, json::Value>>;

impl TryFrom<&ParsedToken> for UserData {
    type Error = JwtError;

    fn try_from(value: &ParsedToken) -> Result<Self, Self::Error> {
        let username = value
            .claims
            .get("sub")
            .ok_or(JwtError::MissingSub)?
            .as_str()
            .ok_or(JwtError::InvalidClaim("sub".to_owned()))?;

        Ok(UserData {
            id: username.to_owned(),
            username: username.to_owned(),
            claims: Claims::new(),
            roles: Roles::new(),
        })
    }
}

impl JwtBearer {
    pub fn new(config: JwtConfig) -> Self {
        Self {
            challenge: "Bearer",
            config: Some(config),
        }
    }

    async fn authenticate_with_header(header: &str, req: &Request<'_>) -> Outcome {
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

        let user = match UserData::try_from(&token) {
            Ok(user) => user,
            Err(err) => {
                return Outcome::Failure(AuthenticationError::InvalidParams(Some(err.boxed())))
            }
        };

        let repository = req.user_repository().await;
        Outcome::Success(repository.user_from_data(user))
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

    async fn authenticate(&self, req: &rocket::Request) -> Outcome {
        for header in req.headers().get("Authorization") {
            match (Self::authenticate_with_header(header, req)).await {
                Outcome::Success(user) => return Outcome::Success(user),
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
pub enum JwtError {
    #[error("Missing sub claim in JWT")]
    MissingSub,

    #[error("Invalid claim value")]
    InvalidClaim(String),
}
