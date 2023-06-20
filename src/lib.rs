pub mod auth;
pub mod config;
pub mod policy;
pub mod scheme;

pub trait RocketExt {
    fn add_identity(self, config: config::Config) -> Self;
}

impl RocketExt for rocket::Rocket<rocket::Build> {
    fn add_identity(self, config: config::Config) -> Self {
        self
            .manage(config)
            .attach(scheme::challenger::Challenger)
    }
}