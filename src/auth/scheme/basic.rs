use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rocket::Request;

use crate::util::Boxable;

use super::prelude::*;

pub struct Basic {
    challenge: String,
}

impl Basic {
    pub fn new(realm: &str) -> Self {
        Self {
            challenge: format!(r#"Basic realm="{}", charset="UTF-8""#, realm),
        }
    }

    async fn authenticate_with_header(header: &str, req: &Request<'_>) -> Outcome {
        // We expect a Basic scheme
        let Some(credentials) = header.strip_prefix("Basic ") else {
            return Outcome::Forward(());
        };

        let credentials = credentials.trim();

        let credentials = match BASE64.decode(credentials) {
            Ok(creds) => creds,
            Err(err) => {
                return Outcome::Failure(AuthenticationError::InvalidParams(Some(err.boxed())))
            }
        };

        let credentials = match String::from_utf8(credentials) {
            Ok(creds) => creds,
            Err(err) => {
                return Outcome::Failure(AuthenticationError::InvalidParams(Some(err.boxed())))
            }
        };

        let Some((username, pass)) = credentials.split_once(':') else {
            return Outcome::Failure(AuthenticationError::Unauthenticated);
        };

        let repository = req.user_repository().await;

        match repository.login(username, pass).await {
            Ok(user) => Outcome::Success(user),
            Err(err) => return Outcome::Failure(err.into()),
        }
    }
}

#[rocket::async_trait]
impl AuthenticationScheme for Basic {
    async fn authenticate(&self, req: &rocket::Request) -> Outcome {
        for header in req.headers().get("Authorization") {
            match (Basic::authenticate_with_header(header, req)).await {
                Outcome::Success(user) => return Outcome::Success(user),
                Outcome::Failure(err) => return Outcome::Failure(err),
                Outcome::Forward(()) => {}
            };
        }

        // No Authorization headers, we cannot handle the request
        Outcome::Forward(())
    }

    fn challenge_header(&self) -> String {
        self.challenge.clone()
    }
}
