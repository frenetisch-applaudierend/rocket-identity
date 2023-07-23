use std::borrow::Cow;

use rocket::{
    http::Cookie,
    serde::{Deserialize, Serialize, json::serde_json},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct SessionData {
    pub username: String,
}

impl SessionData {
    pub fn into_cookie(self, name: impl Into<Cow<'static, str>>) -> Cookie<'static> {
        Cookie::new(name, serde_json::to_string(&self).expect("This should never fail"))
    }
}

impl<'a> TryFrom<Cookie<'a>> for SessionData {
    type Error = serde_json::Error;

    fn try_from(cookie: Cookie<'a>) -> Result<Self, Self::Error> {
        serde_json::from_str(cookie.value())
    }
}
