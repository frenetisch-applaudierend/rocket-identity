use crate::schemes::impls::prelude::*;

#[derive(Debug)]
pub struct CookieScheme {
    cookie_name: String,
}

impl CookieScheme {
    pub fn new(cookie_name: String) -> Self {
        Self { cookie_name }
    }
}

impl Default for CookieScheme {
    fn default() -> Self {
        Self::new("rocket_identity".to_string())
    }
}

#[rocket::async_trait]
impl AuthenticationScheme for CookieScheme {
    fn name(&self) -> String {
        format!("Cookie(name={})", self.cookie_name)
    }

    async fn authenticate(&self, req: &rocket::Request) -> Outcome {
        let _repository = req.user_repository();
        let cookies = req.cookies();

        let Some(_session_cookie) = cookies.get_private(&self.cookie_name) else {
            return Outcome::Forward(());
        };

        todo!()
    }

    async fn challenge(&self, res: &mut rocket::Response) {
        res.adjoin_header(rocket::http::Header::new("WWW-Authenticate", "Cookie"));
    }
}
