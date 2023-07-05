use std::collections::{HashMap, HashSet};

use jsonwebtoken::{decode, Algorithm, TokenData, Validation};
use rocket::{
    serde::json::{self, Value},
    Request,
};

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
        let token_claims = &value.claims;
        let username = read_sub(token_claims)?;
        let roles = read_roles(token_claims)?;

        Ok(UserData {
            id: username.clone(),
            username: username,
            claims: Claims::new(),
            roles,
        })
    }
}

fn read_sub(token_claims: &HashMap<String, Value>) -> Result<String, JwtError> {
    token_claims
        .get("sub")
        .ok_or(JwtError::MissingSub)?
        .as_str()
        .ok_or(JwtError::InvalidClaim("sub".to_owned()))
        .map(|s| s.to_owned())
}

fn read_roles(token_claims: &HashMap<String, Value>) -> Result<Roles, JwtError> {
    let Some(roles) = token_claims.get("roles") else {
        return Ok(Roles::new())
    };

    let roles = roles
        .as_array()
        .ok_or(JwtError::InvalidClaim("roles".to_owned()))?;

    let roles = roles
        .iter()
        .map(|role| {
            role.as_str()
                .map_or(Err(JwtError::InvalidClaim("roles".to_owned())), |r| {
                    Ok(r.to_owned())
                })
        })
        .collect::<Result<HashSet<_>, _>>()?;

    Ok(Roles::from_inner(roles))
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

#[derive(Clone, Debug, thiserror::Error)]
pub enum JwtError {
    #[error("Missing sub claim in JWT")]
    MissingSub,

    #[error("Invalid claim value")]
    InvalidClaim(String),
}
