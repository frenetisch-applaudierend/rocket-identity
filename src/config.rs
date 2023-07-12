use crate::{
    auth::hasher::{Argon2PasswordHasher, PasswordHasher},
    auth::{AuthenticationScheme, MissingAuthPolicy},
    persistence::UserStore,
};

pub struct Config<S, H>
where
    S: UserStore,
    H: PasswordHasher,
{
    user_store: Option<S>,
    password_hasher: Option<H>,
    missing_auth_policy: MissingAuthPolicy,
    auth_schemes: Vec<Box<dyn AuthenticationScheme>>,
}

pub trait ConfigurationProvider: Send + Sync {
    type UserStore: UserStore;
    type PasswordHasher: PasswordHasher;

    fn user_store(&mut self) -> Option<Self::UserStore>;

    fn password_hasher(&mut self) -> Option<Self::PasswordHasher>;

    fn missing_auth_policy(&mut self) -> MissingAuthPolicy;

    fn auth_schemes(&mut self) -> Vec<Box<dyn AuthenticationScheme>>;
}

impl<S> Config<S, Argon2PasswordHasher>
where
    S: UserStore,
{
    pub fn new(user_store: S) -> Self {
        Self {
            user_store: Some(user_store),
            password_hasher: Some(Argon2PasswordHasher::new()),
            missing_auth_policy: MissingAuthPolicy::Fail,
            auth_schemes: Vec::new(),
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
            missing_auth_policy: self.missing_auth_policy,
            auth_schemes: self.auth_schemes,
        }
    }

    pub fn with_password_hasher<H2: PasswordHasher>(self, password_hasher: H2) -> Config<S, H2> {
        Config {
            user_store: self.user_store,
            password_hasher: Some(password_hasher),
            missing_auth_policy: self.missing_auth_policy,
            auth_schemes: self.auth_schemes,
        }
    }

    pub fn with_missing_auth_policy(self, missing_auth_policy: MissingAuthPolicy) -> Config<S, H> {
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

    fn missing_auth_policy(&mut self) -> MissingAuthPolicy {
        self.missing_auth_policy
    }

    fn auth_schemes(&mut self) -> Vec<Box<dyn AuthenticationScheme>> {
        std::mem::take(&mut self.auth_schemes)
    }
}
