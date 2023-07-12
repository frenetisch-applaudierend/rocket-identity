use crate::{
    auth::hasher::{Argon2PasswordHasher, PasswordHasher},
    auth::{AuthenticationScheme, MissingAuthPolicy},
    persistence::UserStore,
};

pub struct Config {
    pub(crate) user_store: Box<dyn UserStore>,
    pub(crate) password_hasher: Box<dyn PasswordHasher>,
    pub(crate) missing_auth_policy: MissingAuthPolicy,
    pub(crate) auth_schemes: Vec<Box<dyn AuthenticationScheme>>,
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

    pub fn with_user_store(self, user_store: impl UserStore + 'static) -> Config {
        Config {
            user_store: Box::new(user_store),
            password_hasher: self.password_hasher,
            missing_auth_policy: self.missing_auth_policy,
            auth_schemes: self.auth_schemes,
        }
    }

    pub fn with_password_hasher(self, password_hasher: impl PasswordHasher + 'static) -> Config {
        Config {
            user_store: self.user_store,
            password_hasher: Box::new(password_hasher),
            missing_auth_policy: self.missing_auth_policy,
            auth_schemes: self.auth_schemes,
        }
    }

    pub fn with_missing_auth_policy(self, missing_auth_policy: MissingAuthPolicy) -> Config {
        Config {
            user_store: self.user_store,
            password_hasher: self.password_hasher,
            missing_auth_policy,
            auth_schemes: self.auth_schemes,
        }
    }

    pub fn add_scheme(mut self, scheme: impl AuthenticationScheme + 'static) -> Self {
        let boxed: Box<dyn AuthenticationScheme> = Box::new(scheme);
        self.auth_schemes.push(boxed);
        self
    }
}
