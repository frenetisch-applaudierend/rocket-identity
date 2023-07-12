use rocket::{
    fairing::{self, Fairing, Info, Kind},
    http::Status,
    Build, Rocket,
};
use tokio::sync::RwLock;

use crate::{
    auth::{hasher::PasswordHasher, scheme::AuthenticationSchemes, UserRepository},
    config::{Config, ConfigurationProvider},
    persistence::UserStore,
};

pub struct RocketIdentity<C: ConfigurationProvider> {
    config: RwLock<C>,
}

impl<S, H> RocketIdentity<Config<S, H>>
where
    S: UserStore,
    H: PasswordHasher,
{
    pub fn fairing(config: Config<S, H>) -> Self {
        Self {
            config: RwLock::new(config),
        }
    }
}

#[rocket::async_trait]
impl<C: ConfigurationProvider + 'static> Fairing for RocketIdentity<C> {
    fn info(&self) -> Info {
        Info {
            name: "Rocket Identity",
            kind: Kind::Ignite | Kind::Liftoff | Kind::Response | Kind::Singleton,
        }
    }

    /// On ignition we verify the configuration and setup the necessary managed state.
    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let mut rocket = rocket;
        let mut config = self.config.write().await;

        // Load user store from config
        let Some(user_store) = config.user_store() else {
            log::error!("No user store configured");
            return Err(rocket);
        };

        // Load password hasher from config
        let Some(password_hasher) = config.password_hasher() else {
            log::error!("No password hasher configured");
            return Err(rocket);
        };

        // Load missing auth policy from config
        let missing_auth_policy = config.missing_auth_policy();

        // Load authentication schemes from config
        let mut auth_schemes = AuthenticationSchemes::new(config.auth_schemes());
        if auth_schemes.is_empty() {
            log::warn!("No authentication schemes configured");
        }

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
            .state::<AuthenticationSchemes>()
            .expect("Missing authentication schemes");

        for scheme in auth_schemes.iter() {
            scheme.challenge(res).await;
        }
    }
}
