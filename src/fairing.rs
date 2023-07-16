use rocket::{
    fairing::{self, Fairing, Info, Kind},
    http::Status,
    log::PaintExt,
    Build, Orbit, Rocket,
};
use tokio::sync::RwLock;

use yansi::Paint;

use crate::{
    auth::{scheme::AuthenticationSchemes, UserRepository},
    config::Config,
};

pub struct RocketIdentity<TUserId: 'static> {
    config: RwLock<Option<Config<TUserId>>>,
}

impl<TUserId> RocketIdentity<TUserId> {
    pub fn fairing(config: Config<TUserId>) -> Self {
        Self {
            config: RwLock::new(Some(config)),
        }
    }
}

#[rocket::async_trait]
impl<TUserId> Fairing for RocketIdentity<TUserId> {
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

        // Create user repository
        let user_repository = UserRepository::new(user_store, password_hasher);

        // Add managed state and return rocket instance
        Ok(rocket
            .manage(user_repository)
            .manage(missing_auth_policy)
            .manage(auth_schemes))
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        rocket::info!("{}{}:", Paint::emoji("🔐 "), Paint::magenta("Identity"));

        // Log authentication schemes
        let auth_schemes = rocket
            .state::<AuthenticationSchemes<TUserId>>()
            .expect("Missing authentication schemes");

        if auth_schemes.is_empty() {
            rocket::warn_!("No authentication schemes configured");
        }

        for scheme in auth_schemes.iter() {
            rocket::info_!("Authentication scheme: {}", scheme.name());
        }
    }

    /// On response we check if the response was 401 Unauthorized and if so we add a
    /// WWW-Authenticate header with the configured authentication schemes.
    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        // Only listen for status 401 Unauthorized
        if res.status() != Status::Unauthorized {
            return;
        }

        // If an existing WWW-Authenticate header is present we just leave it
        // under the assumption, that the request handler wants a specific value.
        if res.headers().contains("WWW-Authenticate") {
            return;
        }

        // Add WWW-Authenticate header for each authentication scheme
        let auth_schemes = req
            .rocket()
            .state::<AuthenticationSchemes<TUserId>>()
            .expect("Missing authentication schemes");

        for scheme in auth_schemes.iter() {
            scheme.challenge(res).await;
        }
    }
}
