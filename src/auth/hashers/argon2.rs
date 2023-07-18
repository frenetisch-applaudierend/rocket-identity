use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::{auth::UserData, util::Result};

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

impl crate::PasswordHasher for Argon2PasswordHasher {
    fn hash_password(&self, _user: &UserData, password: &str) -> Result<Vec<u8>> {
        let salt = SaltString::generate(OsRng);
        let password_hash = self.ctx.hash_password(password.as_bytes(), &salt)?;

        Ok(password_hash.to_string().into_bytes())
    }

    fn verify_password(
        &self,
        _user: &UserData,
        password_hash: &[u8],
        password: &str,
    ) -> crate::util::Result<bool> {
        let password_hash = std::str::from_utf8(password_hash)?;
        let password_hash = PasswordHash::new(password_hash)?;

        Ok(self
            .ctx
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok())
    }
}
