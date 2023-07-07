use crate::{
    auth::hasher::PasswordHasher,
    auth::{scheme::AuthenticationScheme, DefaultUserRepository, UserRepository},
    persistence::UserStore,
};

pub struct Config {
    pub user_repository: Box<dyn UserRepository>,
    pub auth_schemes: Vec<Box<dyn AuthenticationScheme>>,
}

impl Config {
    pub fn new(
        user_store: impl UserStore + 'static,
        password_hasher: impl PasswordHasher + 'static,
    ) -> Self {
        Self {
            user_repository: Box::new(DefaultUserRepository {
                store: user_store,
                hasher: password_hasher,
            }),
            auth_schemes: Vec::new(),
        }
    }

    pub fn add_scheme(mut self, scheme: impl AuthenticationScheme + 'static) -> Self {
        let boxed: Box<dyn AuthenticationScheme> = Box::new(scheme);
        self.auth_schemes.push(boxed);
        self
    }
}
