pub mod auth;
pub mod config;
pub mod persistence;
pub mod util;

use auth::scheme::AuthenticationSchemes;

pub trait RocketExt {
    fn add_identity(self, config: config::Config) -> Self;
}

impl RocketExt for rocket::Rocket<rocket::Build> {
    fn add_identity(mut self, config: config::Config) -> Self {
        let user_repository = config.user_repository;
        let password_hasher = config.password_hasher;
        let mut auth_schemes = AuthenticationSchemes::new(config.auth_schemes);

        for scheme in auth_schemes.iter_mut() {
            self = scheme.setup(self);
        }

        self.manage(user_repository)
            .manage(password_hasher)
            .manage(auth_schemes)
            .attach(auth::scheme::challenger::Challenger)
    }
}
