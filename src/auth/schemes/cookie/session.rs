use rocket::{
    http::{Cookie, CookieJar},
    request::{FromRequest, Outcome},
    serde::{json::serde_json, Deserialize, Serialize},
    Request,
};

use crate::User;

use super::CookieScheme;

#[derive(Debug)]
pub struct CookieSession<'r> {
    cookie_jar: &'r CookieJar<'r>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SessionData {
    pub username: String,
}

impl<'r> CookieSession<'r> {
    pub fn sign_in(&self, user: &User) {
        self.sign_in_with_cookie(user, CookieScheme::default_cookie_name())
    }

    pub fn sign_in_with_cookie(&self, user: &User, cookie_name: &str) {
        let session = SessionData {
            username: user.username.clone(),
        };
        let session = serde_json::to_string(&session).expect("This should never fail");

        self.cookie_jar
            .add_private(Cookie::new(cookie_name.to_string(), session));
    }
}

// implement FromRequest for CookieSession
#[rocket::async_trait]
impl<'r> FromRequest<'r> for CookieSession<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie_jar = req.cookies();

        Outcome::Success(CookieSession { cookie_jar })
    }
}
