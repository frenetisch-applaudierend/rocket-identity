use scheme::AuthenticationSchemes;

pub mod auth;
pub mod config;
pub mod persistence;
pub mod policy;
pub mod scheme;
pub mod util;

pub trait RocketExt {
    fn add_identity(self, config: config::Config) -> Self;
}

impl RocketExt for rocket::Rocket<rocket::Build> {
    fn add_identity(self, config: config::Config) -> Self {
        let user_repository = config.user_repository;
        let password_hasher = config.password_hasher;
        let auth_schemes = AuthenticationSchemes(config.auth_schemes);

        self.manage(user_repository)
            .manage(password_hasher)
            .manage(auth_schemes)
            .attach(scheme::challenger::Challenger)
    }
}
