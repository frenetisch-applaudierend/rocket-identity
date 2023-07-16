use crate::auth::scheme::prelude::*;

pub struct Cookie {
    cookie_name: String,
}

impl Cookie {
    pub fn new(cookie_name: String) -> Self {
        Self { cookie_name }
    }
}

#[rocket::async_trait]
impl<TUserId: 'static> AuthenticationScheme<TUserId> for Cookie {
    fn name(&self) -> &'static str {
        "Cookie"
    }

    async fn authenticate(
        &self,
        req: &rocket::Request,
        _user_builder: &UserBuilder,
    ) -> Outcome<TUserId> {
        let _repository = req.user_repository::<TUserId>();
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
