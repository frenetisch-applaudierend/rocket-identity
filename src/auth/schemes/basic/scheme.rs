use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rocket::Request;

use crate::auth::{schemes::prelude::*, LoginError};

#[derive(Debug)]
pub struct Basic {
    realm: String,
    challenge: String,
}

impl Basic {
    pub fn new(realm: &str) -> Self {
        Self {
            realm: realm.to_string(),
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
                log::error!("Failed to decode credentials: {}", err);
                return Outcome::Failure(AuthenticationError::InvalidParams);
            }
        };

        let credentials = match String::from_utf8(credentials) {
            Ok(creds) => creds,
            Err(err) => {
                log::error!("Failed to decode credentials: {}", err);
                return Outcome::Failure(AuthenticationError::InvalidParams);
            }
        };

        let Some((username, pass)) = credentials.split_once(':') else {
            return Outcome::Failure(AuthenticationError::Unauthenticated);
        };

        let repository = req.user_repository();

        match repository.authenticate(username, pass).await {
            Ok(user) => Outcome::Success(user),
            Err(err) => Outcome::Failure(err.into()),
        }
    }
}

#[rocket::async_trait]
impl AuthenticationScheme for Basic {
    fn name(&self) -> String {
        format!("Basic(realm={})", self.realm)
    }

    async fn authenticate(&self, req: &rocket::Request, _user_builder: &UserBuilder) -> Outcome {
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

    async fn challenge(&self, res: &mut rocket::Response) {
        res.adjoin_header(rocket::http::Header::new(
            "WWW-Authenticate",
            self.challenge.clone(),
        ));
    }
}

impl From<LoginError> for AuthenticationError {
    fn from(err: LoginError) -> Self {
        match err {
            LoginError::UserNotFound => AuthenticationError::Unauthenticated,
            LoginError::MissingPassword => AuthenticationError::Unauthenticated,
            LoginError::IncorrectPassword => AuthenticationError::Unauthenticated,
            LoginError::Other => AuthenticationError::Other,
        }
    }
}
