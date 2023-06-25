use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rocket::http::Status;
use thiserror::Error;

use crate::auth::UserRepository;

use super::{AuthenticationScheme, Outcome};

pub struct Basic {
    challenge: String,
}

impl Basic {
    pub fn new(realm: &str) -> Self {
        Self {
            challenge: format!(r#"Basic realm="{}", charset="UTF-8""#, realm),
        }
    }
}

#[rocket::async_trait]
impl AuthenticationScheme for Basic {
    async fn autenticate(&self, req: &rocket::Request) -> Outcome {
        for header in req.headers().get("Authorization") {
            // We expect a Basic scheme
            let Some(credentials) = header.strip_prefix("Basic ") else {
                continue;
            };

            let credentials = credentials.trim();

            let credentials = match BASE64.decode(credentials) {
                Ok(creds) => creds,
                Err(err) => return Outcome::Failure((Status::BadRequest, Box::new(err))),
            };

            let credentials = match String::from_utf8(credentials) {
                Ok(creds) => creds,
                Err(err) => return Outcome::Failure((Status::BadRequest, Box::new(err))),
            };

            let Some((user, pass)) = credentials.split_once(':') else {
                return Outcome::Failure((Status::BadRequest, Box::new(BasicError::InvalidUserPass)))
            };

            let authenticator = req
                .guard::<UserRepository>()
                .await
                .expect("Authenticator should never fail");

            let user = match authenticator.login(user, pass).await {
                Ok(user) => user,
                Err(err) => return Outcome::Failure((Status::Unauthorized, Box::new(err))),
            };

            return Outcome::Success(user);
        }

        // No Authorization headers, we cannot handle the request
        Outcome::Forward(())
    }

    fn challenge(&self) -> String {
        self.challenge.clone()
    }
}

#[derive(Error, Debug)]
enum BasicError {
    #[error("Invalid user-pass string provided")]
    InvalidUserPass,
}
