use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rocket::Request;

use crate::{
    auth::{User, UserRepository},
    util::Boxable,
};

use super::{AuthenticationError, AuthenticationScheme, Outcome};

pub struct Basic {
    challenge: String,
}

impl Basic {
    pub fn new(realm: &str) -> Self {
        Self {
            challenge: format!(r#"Basic realm="{}", charset="UTF-8""#, realm),
        }
    }

    async fn authenticate_with_header(header: &str, user: &mut User, req: &Request<'_>) -> Outcome {
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

        let authenticator = req
            .guard::<UserRepository>()
            .await
            .expect("Authenticator should never fail");

        *user = match authenticator.login(username, pass).await {
            Ok(user) => user,
            Err(err) => return Outcome::Failure(err.into()),
        };

        Outcome::Success(())
    }
}

#[rocket::async_trait]
impl AuthenticationScheme for Basic {
    async fn authenticate(&self, user: &mut User, req: &rocket::Request) -> Outcome {
        for header in req.headers().get("Authorization") {
            match (Basic::authenticate_with_header(header, user, req)).await {
                Outcome::Success(()) => return Outcome::Success(()),
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
