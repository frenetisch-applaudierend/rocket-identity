use crate::scheme::{AuthenticationScheme, Outcome};

use super::JwtTokenProvider;

pub struct JwtBearer {
    challenge: String,
}

impl JwtBearer {
    pub fn new(realm: &str, secret: &str) -> Self {
        Self {
            challenge: format!(r#"Bearer realm="{}""#, realm),
        }
    }
}

#[rocket::async_trait]
impl AuthenticationScheme for JwtBearer {
    fn setup(&self, rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        rocket.manage(JwtTokenProvider)
    }

    async fn autenticate(&self, _req: &rocket::Request) -> Outcome {
        todo!()
    }

    fn challenge(&self) -> String {
        self.challenge.clone()
    }
}
