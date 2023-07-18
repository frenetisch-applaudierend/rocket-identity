use crate::auth::schemes::prelude::*;

#[derive(Debug)]
pub struct Cookie {
    cookie_name: String,
}

impl Cookie {
    pub fn new(cookie_name: String) -> Self {
        Self { cookie_name }
    }
}

impl Default for Cookie {
    fn default() -> Self {
        Self::new("rocket_identity".to_string())
    }
}

#[rocket::async_trait]
impl AuthenticationScheme for Cookie {
    fn name(&self) -> String {
        format!("Cookie(name={})", self.cookie_name)
    }

    async fn authenticate(&self, req: &rocket::Request, _user_builder: &UserBuilder) -> Outcome {
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
