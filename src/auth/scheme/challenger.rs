use super::AuthenticationSchemes;

pub(crate) struct Challenger;

#[rocket::async_trait]
impl rocket::fairing::Fairing for Challenger {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Authentication scheme challenge provider",
            kind: rocket::fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        // Listen for status 401
        if res.status() != rocket::http::Status::Unauthorized {
            return;
        }

        // If an existing WWW-Authenticate header is present we just leave it
        if res.headers().contains("WWW-Authenticate") {
            return;
        }

        let auth_schemes = req
            .rocket()
            .state::<AuthenticationSchemes>()
            .expect("Missing configuration");

        for scheme in auth_schemes.iter() {
            res.adjoin_header(rocket::http::Header::new(
                "WWW-Authenticate",
                scheme.challenge_header(),
            ));
        }
    }
}