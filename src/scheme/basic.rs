use base64::Engine;
use rocket::http::Status;

use crate::auth::User;

use super::{AuthenticationScheme, Outcome};

pub struct Basic;

#[rocket::async_trait]
impl AuthenticationScheme for Basic {
    async fn autenticate(&self, req: &rocket::Request) -> Outcome {
        for header in req.headers().get("Authorization") {
            println!("Checking {}", header);
            
            // We expect a Basic scheme
            let Some(credentials) = header.strip_prefix("Basic ") else {
                continue;
            };

            let credentials = credentials.trim();

            let credentials = match base64::engine::general_purpose::STANDARD.decode(credentials) {
                Ok(creds) => creds,
                Err(err) => return Outcome::Failure((Status::BadRequest, Box::new(err))),
            };

            let credentials = match String::from_utf8(credentials) {
                Ok(creds) => creds,
                Err(err) => return Outcome::Failure((Status::BadRequest, Box::new(err))),
            };

            return Outcome::Success(User {
                user_name: credentials,
            });
        }

        // no headers we cannot handle the request
        Outcome::Forward(())
    }
}
