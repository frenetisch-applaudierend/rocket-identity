use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    PasswordHasher as _, PasswordVerifier as _,
};

use crate::hashers::impls::prelude::*;

#[derive(Default, Clone)]
pub struct Argon2PasswordHasher {
    ctx: argon2::Argon2<'static>,
}

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self {
            ctx: argon2::Argon2::default(),
        }
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash_password(&self, _user: &User, password: &str) -> Result<PasswordHash> {
        let salt = SaltString::generate(OsRng);
        let password_hash = self
            .ctx
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash.into())
    }

    fn verify_password(
        &self,
        _user: &User,
        password_hash: &PasswordHash,
        password: &str,
    ) -> crate::util::Result<bool> {
        let password_hash = std::str::from_utf8(password_hash.as_bytes())?;
        let password_hash = argon2::PasswordHash::new(password_hash)?;

        Ok(self
            .ctx
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok())
    }
}
