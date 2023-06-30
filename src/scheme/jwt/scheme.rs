use std::collections::HashMap;

use jsonwebtoken::{decode, Algorithm, TokenData, Validation};
use rocket::{http::Status, serde::json};

use crate::{
    auth::User,
    scheme::{AuthenticationScheme, Outcome},
};

use super::JwtConfig;

pub struct JwtBearer {
    challenge: &'static str,
    config: Option<JwtConfig>,
}

type Token = TokenData<HashMap<String, json::Value>>;

impl JwtBearer {
    pub fn new(config: JwtConfig) -> Self {
        Self {
            challenge: "Bearer",
            config: Some(config),
        }
    }

    fn user_from_token(_token: &Token) -> User {
        todo!()
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

    async fn autenticate(&self, req: &rocket::Request) -> Outcome {
        for header in req.headers().get("Authorization") {
            // We expect a Basic scheme
            let Some(token) = header.strip_prefix("Bearer ") else {
                continue;
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
                Err(err) => return Outcome::Failure((Status::BadRequest, Box::new(err))),
            };

            let user = Self::user_from_token(&token);

            return Outcome::Success(user);
        }

        // No Authorization headers, we cannot handle the request
        Outcome::Forward(())
    }

    fn challenge(&self) -> String {
        self.challenge.to_string()
    }
}
