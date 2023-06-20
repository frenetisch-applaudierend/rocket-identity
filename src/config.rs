use crate::scheme::AuthenticationScheme;

pub struct Config {
    pub auth_schemes: Vec<Box<dyn AuthenticationScheme + Send + Sync>>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            auth_schemes: Vec::new(),
        }
    }

    pub fn add_scheme(mut self, scheme: impl AuthenticationScheme + Send + Sync + 'static) -> Self {
        let boxed: Box<dyn AuthenticationScheme + Send + Sync> = Box::new(scheme);
        self.auth_schemes.push(boxed);
        self
    }
}