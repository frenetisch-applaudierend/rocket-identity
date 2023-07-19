use crate::schemes::impls::prelude::*;

use super::session_data::SessionData;

#[derive(Debug)]
pub struct CookieScheme {
    cookie_name: String,
}

impl CookieScheme {
    pub fn default_cookie_name() -> &'static str {
        "rocket_identity"
    }

    pub fn new(cookie_name: impl Into<String>) -> Self {
        Self {
            cookie_name: cookie_name.into(),
        }
    }
}

impl Default for CookieScheme {
    fn default() -> Self {
        Self::new(Self::default_cookie_name())
    }
}

#[rocket::async_trait]
impl AuthenticationScheme for CookieScheme {
    fn name(&self) -> String {
        format!("Cookie(name={})", self.cookie_name)
    }

    async fn authenticate(&self, req: &rocket::Request) -> Outcome {
        let users = req.user_repository();
        let cookies = req.cookies();

        let Some(session_cookie) = cookies.get_private(&self.cookie_name) else {
            return Outcome::Forward(());
        };

        let session_data = match SessionData::try_from(session_cookie) {
            Ok(session_data) => session_data,
            Err(e) => {
                log::error!("Failed to deserialize session data: {}", e);
                return Outcome::Failure(AuthenticationError::InvalidParams);
            }
        };

        let user = match users.find_by_username(&session_data.username).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                log::error!("Failed to find user");
                return Outcome::Failure(AuthenticationError::Unauthenticated);
            }
            Err(e) => {
                log::error!("Failed to find user: {}", e);
                return Outcome::Failure(AuthenticationError::Other);
            }
        };

        Outcome::Success(user)
    }

    async fn challenge(&self, res: &mut rocket::Response) {
        res.adjoin_header(rocket::http::Header::new("WWW-Authenticate", "Cookie"));
    }
}
