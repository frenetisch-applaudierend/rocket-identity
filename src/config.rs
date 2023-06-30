use crate::{auth::hasher::PasswordHasher, persistence::UserStore, scheme::AuthenticationScheme};

pub struct Config {
    pub user_repository: Box<dyn UserStore>,
    pub password_hasher: Box<dyn PasswordHasher>,
    pub auth_schemes: Vec<Box<dyn AuthenticationScheme>>,
}

impl Config {
    pub fn new(
        repository: impl UserStore + 'static,
        password_hasher: impl PasswordHasher + 'static,
    ) -> Self {
        Self {
            user_repository: Box::new(repository),
            password_hasher: Box::new(password_hasher),
            auth_schemes: Vec::new(),
        }
    }

    pub fn add_scheme(mut self, scheme: impl AuthenticationScheme + 'static) -> Self {
        let boxed: Box<dyn AuthenticationScheme> = Box::new(scheme);
        self.auth_schemes.push(boxed);
        self
    }
}
