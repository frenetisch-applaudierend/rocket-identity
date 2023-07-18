use crate::{
    hashers::Argon2PasswordHasher,
    persistence::UserStore,
    {AuthenticationScheme, Identity, PasswordHasher},
};

pub struct Config {
    pub(crate) user_store: Box<dyn UserStore>,
    pub(crate) password_hasher: Box<dyn PasswordHasher>,
    pub(crate) missing_auth_policy: MissingAuthPolicy,
    pub(crate) auth_schemes: Vec<Box<dyn AuthenticationScheme>>,
}

#[derive(Debug, Clone, Copy)]
pub enum MissingAuthPolicy {
    Fail,
    Forward,
}

impl Config {
    pub fn new(user_store: impl UserStore + 'static) -> Self {
        Self {
            user_store: Box::new(user_store),
            password_hasher: Box::new(Argon2PasswordHasher::new()),
            missing_auth_policy: MissingAuthPolicy::Fail,
            auth_schemes: Vec::new(),
        }
    }

    pub fn with_password_hasher(self, password_hasher: impl PasswordHasher + 'static) -> Self {
        Self {
            user_store: self.user_store,
            password_hasher: Box::new(password_hasher),
            missing_auth_policy: self.missing_auth_policy,
            auth_schemes: self.auth_schemes,
        }
    }

    pub fn with_missing_auth_policy(self, missing_auth_policy: MissingAuthPolicy) -> Self {
        Self {
            user_store: self.user_store,
            password_hasher: self.password_hasher,
            missing_auth_policy,
            auth_schemes: self.auth_schemes,
        }
    }

    pub fn add_scheme(mut self, scheme: impl AuthenticationScheme + 'static) -> Self {
        self.auth_schemes.push(Box::new(scheme));
        self
    }
}

impl Identity {
    pub fn config(user_store: impl UserStore + 'static) -> Config {
        Config::new(user_store)
    }
}
