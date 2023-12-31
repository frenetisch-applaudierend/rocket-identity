use rocket::{
    fairing::{self, Fairing, Info, Kind},
    http::Status,
    log::PaintExt,
    Build, Orbit, Rocket,
};
use tokio::sync::RwLock;

use yansi::Paint;

use crate::{config::Config, schemes::AuthenticationSchemes, Identity, Services};

impl Identity {
    pub fn fairing(config: Config) -> Self {
        Self {
            config: RwLock::new(Some(config)),
        }
    }
}

#[rocket::async_trait]
impl Fairing for Identity {
    fn info(&self) -> Info {
        Info {
            name: "Identity",
            kind: Kind::Ignite | Kind::Liftoff | Kind::Response | Kind::Singleton,
        }
    }

    /// On ignition we verify the configuration and setup the necessary managed state.
    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let mut rocket = rocket;
        let config = self.config.write().await.take().expect("Missing config");

        let user_store = config.user_store;
        let password_hasher = config.password_hasher;
        let missing_auth_policy = config.missing_auth_policy;
        let mut auth_schemes = AuthenticationSchemes::new(config.auth_schemes);

        // Allow authentication schemes to setup themselves
        for scheme in auth_schemes.iter_mut() {
            rocket = scheme.setup(rocket);
        }

        // Add user store if configured
        if let Some(user_store) = user_store {
            rocket = rocket.manage(user_store);
        }

        // Add password hasher if configured
        if let Some(password_hasher) = password_hasher {
            rocket = rocket.manage(password_hasher);
        }

        // Add missing auth policy
        rocket = rocket.manage(missing_auth_policy);

        // Add auth schemes
        rocket = rocket.manage(auth_schemes);

        // Return the configured rocket instance
        Ok(rocket)
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        rocket::info!("{}{}:", Paint::emoji("🔐 "), Paint::magenta("Identity"));

        // Log authentication schemes
        let auth_schemes = rocket.authentication_schemes();

        if auth_schemes.is_empty() {
            rocket::warn_!("No authentication schemes configured");
        }

        for scheme in auth_schemes.iter() {
            rocket::info_!("Authentication scheme: {}", Paint::default(scheme.name()));
        }
    }

    /// On response we check if the response was 401 Unauthorized and if so we add a
    /// WWW-Authenticate header with the configured authentication schemes.
    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        // Only listen for status 401 Unauthorized
        if res.status() != Status::Unauthorized {
            return;
        }

        // Add WWW-Authenticate header for each authentication scheme
        let auth_schemes = req.authentication_schemes();

        for scheme in auth_schemes.iter() {
            scheme.challenge(res).await;
        }
    }
}
