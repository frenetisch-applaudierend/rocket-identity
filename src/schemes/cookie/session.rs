use std::borrow::Cow;

use rocket::{
    http::CookieJar,
    request::{FromRequest, Outcome},
    Request,
};

use crate::User;

use super::{session_data::SessionData, CookieScheme};

#[derive(Debug)]
pub struct CookieSession<'r> {
    cookie_jar: &'r CookieJar<'r>,
}

impl<'r> CookieSession<'r> {
    pub fn sign_in(&self, user: &User) {
        self.sign_in_with_cookie(user, CookieScheme::default_cookie_name())
    }

    pub fn sign_in_with_cookie(&self, user: &User, cookie_name: impl Into<Cow<'static, str>>) {
        let session = SessionData {
            username: user.username.clone(),
        };

        self.cookie_jar
            .add_private(session.into_cookie(cookie_name));
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
