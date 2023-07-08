use crate::{
    auth::hasher::{Argon2PasswordHasher, PasswordHasher},
    auth::scheme::AuthenticationScheme,
    persistence::UserStore,
};

pub struct Config<S, H>
where
    S: UserStore,
    H: PasswordHasher,
{
    user_store: Option<S>,
    password_hasher: Option<H>,
    auth_schemes: Vec<Box<dyn AuthenticationScheme>>,
    initializer: Option<Box<dyn RocketIdentityInitializer>>,
}

#[rocket::async_trait]
pub trait RocketIdentityInitializer: Send + Sync {
    async fn initialize(&self, rocket: &rocket::Rocket<rocket::Orbit>);
}

pub trait ConfigurationProvider: Send + Sync {
    type UserStore: UserStore;
    type PasswordHasher: PasswordHasher;

    fn user_store(&mut self) -> Option<Self::UserStore>;

    fn password_hasher(&mut self) -> Option<Self::PasswordHasher>;

    fn auth_schemes(&mut self) -> Vec<Box<dyn AuthenticationScheme>>;

    fn initializer(&mut self) -> Option<Box<dyn RocketIdentityInitializer>>;
}

impl<S> Config<S, Argon2PasswordHasher>
where
    S: UserStore,
{
    pub fn new(user_store: S) -> Self {
        Self {
            user_store: Some(user_store),
            password_hasher: Some(Argon2PasswordHasher::new()),
            auth_schemes: Vec::new(),
            initializer: None,
        }
    }
}

impl<S, H> Config<S, H>
where
    S: UserStore,
    H: PasswordHasher,
{
    pub fn with_user_store<S2: UserStore>(self, user_store: S2) -> Config<S2, H> {
        Config {
            user_store: Some(user_store),
            password_hasher: self.password_hasher,
            auth_schemes: self.auth_schemes,
            initializer: self.initializer,
        }
    }

    pub fn with_password_hasher<H2: PasswordHasher>(self, password_hasher: H2) -> Config<S, H2> {
        Config {
            user_store: self.user_store,
            password_hasher: Some(password_hasher),
            auth_schemes: self.auth_schemes,
            initializer: self.initializer,
        }
    }

    pub fn with_initializer(self, initializer: impl RocketIdentityInitializer + 'static) -> Self {
        Self {
            user_store: self.user_store,
            password_hasher: self.password_hasher,
            auth_schemes: self.auth_schemes,
            initializer: Some(Box::new(initializer)),
        }
    }

    pub fn add_scheme(mut self, scheme: impl AuthenticationScheme + 'static) -> Self {
        let boxed: Box<dyn AuthenticationScheme> = Box::new(scheme);
        self.auth_schemes.push(boxed);
        self
    }
}

impl<S, H> ConfigurationProvider for Config<S, H>
where
    S: UserStore,
    H: PasswordHasher,
{
    type UserStore = S;
    type PasswordHasher = H;

    fn user_store(&mut self) -> Option<Self::UserStore> {
        self.user_store.take()
    }

    fn password_hasher(&mut self) -> Option<Self::PasswordHasher> {
        self.password_hasher.take()
    }

    fn auth_schemes(&mut self) -> Vec<Box<dyn AuthenticationScheme>> {
        std::mem::take(&mut self.auth_schemes)
    }

    fn initializer(&mut self) -> Option<Box<dyn RocketIdentityInitializer>> {
        self.initializer.take()
    }
}

#[rocket::async_trait]
impl<T, F> RocketIdentityInitializer for T
where
    T: Fn(&rocket::Rocket<rocket::Orbit>) -> F + Send + Sync + 'static,
    F: std::future::Future<Output = ()> + Send + Sync + 'static,
{
    async fn initialize(&self, rocket: &rocket::Rocket<rocket::Orbit>) {
        self(rocket);
    }
}
