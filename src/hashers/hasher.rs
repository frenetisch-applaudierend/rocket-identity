use crate::{util::Result, User};

pub trait PasswordHasher: Send + Sync + core::fmt::Debug + 'static {
    fn hash_password(&self, user: &User, password: &str) -> Result<PasswordHash>;

    fn verify_password(
        &self,
        user: &User,
        password_hash: &PasswordHash,
        password: &str,
    ) -> Result<bool>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(Vec<u8>);

impl PasswordHash {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl<T: Into<Vec<u8>>> From<T> for PasswordHash {
    fn from(bytes: T) -> Self {
        Self(bytes.into())
    }
}
