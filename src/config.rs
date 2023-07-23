use std::sync::Arc;

use crate::{
    hashers::{argon2::Argon2PasswordHasher, PasswordHasher},
    schemes::AuthenticationScheme,
    stores::UserStore,
    Identity,
};

#[derive(Debug)]
pub struct Config {
    pub(crate) user_store: Option<Box<dyn UserStore>>,
    pub(crate) password_hasher: Option<Arc<dyn PasswordHasher>>,
    pub(crate) auth_schemes: Vec<Box<dyn AuthenticationScheme>>,
    pub(crate) missing_auth_policy: MissingAuthPolicy,
}

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    config: Option<Config>,
}

#[derive(Debug, Clone, Copy)]
pub enum MissingAuthPolicy {
    Fail,
    Forward,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self { config: None }
    }

    fn config(&mut self) -> &mut Config {
        self.config.get_or_insert_with(|| Config {
            user_store: None,
            password_hasher: Some(Arc::new(Argon2PasswordHasher::new())),
            auth_schemes: Vec::new(),
            missing_auth_policy: MissingAuthPolicy::Fail,
        })
    }

    pub fn with_user_store(&mut self, user_store: impl UserStore) -> &mut Self {
        self.config().user_store = Some(Box::new(user_store));
        self
    }

    pub fn with_password_hasher(&mut self, password_hasher: impl PasswordHasher) -> &mut Self {
        self.config().password_hasher = Some(Arc::new(password_hasher));
        self
    }

    pub fn with_missing_auth_policy(
        &mut self,
        missing_auth_policy: MissingAuthPolicy,
    ) -> &mut Self {
        self.config().missing_auth_policy = missing_auth_policy;
        self
    }

    pub fn add_scheme(&mut self, scheme: impl AuthenticationScheme) -> &mut Self {
        self.config().auth_schemes.push(Box::new(scheme));
        self
    }

    pub fn build(&mut self) -> Config {
        _ = self.config(); // Ensure config is initialized
        self.config
            .take()
            .expect("Config should be initialized here")
    }
}

impl Identity {
    pub fn config() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}
