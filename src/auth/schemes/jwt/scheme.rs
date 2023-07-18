use std::collections::{HashMap, HashSet};

use jsonwebtoken::{decode, Algorithm, TokenData, Validation};
use rocket::{serde::json::Value, Request};

use crate::auth::schemes::prelude::*;

use super::JwtConfig;

#[derive(Debug)]
pub struct JwtBearer {
    challenge: &'static str,
    config: Option<JwtConfig>,
}

type ParsedToken = TokenData<HashMap<String, Value>>;

impl TryFrom<&ParsedToken> for UserData {
    type Error = JwtError;

    fn try_from(value: &ParsedToken) -> Result<Self, Self::Error> {
        let token_claims = &value.claims;
        let username = read_sub(token_claims)?;
        let roles = read_roles(token_claims)?;

        Ok(UserData {
            username,
            claims: Claims::new(),
            roles,
        })
    }
}

fn read_sub(token_claims: &HashMap<String, Value>) -> Result<String, JwtError> {
    let sub = token_claims
        .get("sub")
        .ok_or(JwtError::MissingSub)?
        .as_str()
        .ok_or(JwtError::InvalidClaim("sub".to_owned()))
        .map(|s| s.to_owned())?;

    if sub.is_empty() {
        return Err(JwtError::InvalidClaim("sub".to_owned()));
    }

    Ok(sub)
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

    async fn authenticate_with_header(
        header: &str,
        req: &Request<'_>,
        user_builder: &UserBuilder,
    ) -> Outcome {
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

        let token = match decode::<HashMap<String, Value>>(token, key, &validation) {
            Ok(token) => token,
            Err(err) => {
                log::error!("Failed to decode token: {}", err);
                return Outcome::Failure(AuthenticationError::InvalidParams);
            }
        };

        let user = match UserData::try_from(&token) {
            Ok(user) => user,
            Err(err) => {
                log::error!("Failed to get user data from token: {}", err);
                return Outcome::Failure(AuthenticationError::InvalidParams);
            }
        };

        Outcome::Success(user_builder.build(user))
    }
}

#[rocket::async_trait]
impl AuthenticationScheme for JwtBearer {
    fn name(&self) -> String {
        "JwtBearer".to_owned()
    }

    fn setup(&mut self, rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        rocket.manage(
            self.config
                .take()
                .expect("JwtConfig was already taken or never added"),
        )
    }

    async fn authenticate(&self, req: &rocket::Request, user_builder: &UserBuilder) -> Outcome {
        for header in req.headers().get("Authorization") {
            match (Self::authenticate_with_header(header, req, user_builder)).await {
                Outcome::Success(user) => return Outcome::Success(user),
                Outcome::Failure(err) => return Outcome::Failure(err),
                Outcome::Forward(()) => {}
            }
        }

        // No Authorization headers, we cannot handle the request
        Outcome::Forward(())
    }

    async fn challenge(&self, res: &mut rocket::Response) {
        res.adjoin_header(rocket::http::Header::new(
            "WWW-Authenticate",
            self.challenge,
        ));
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum JwtError {
    #[error("Missing sub claim in JWT")]
    MissingSub,

    #[error("Invalid claim value")]
    InvalidClaim(String),
}
