use crate::{
    auth::{IdentityPasswordHasher, PasswordHasher},
    persistence::UserRepository,
    scheme::AuthenticationScheme,
};

pub struct Config {
    pub user_repository: Box<dyn UserRepository>,
    pub password_hasher: Box<dyn PasswordHasher>,
    pub auth_schemes: Vec<Box<dyn AuthenticationScheme>>,
}

impl Config {
    pub fn new(repository: impl UserRepository + 'static) -> Self {
        let password_hasher = IdentityPasswordHasher;

        Self {
            user_repository: Box::new(repository),
            password_hasher: Box::new(password_hasher),
            auth_schemes: Vec::new(),
        }
    }

    pub fn use_password_hasher(mut self, hasher: impl PasswordHasher + 'static) -> Self {
        self.password_hasher = Box::new(hasher);
        self
    }

    pub fn add_scheme(mut self, scheme: impl AuthenticationScheme + 'static) -> Self {
        let boxed: Box<dyn AuthenticationScheme> = Box::new(scheme);
        self.auth_schemes.push(boxed);
        self
    }
}
