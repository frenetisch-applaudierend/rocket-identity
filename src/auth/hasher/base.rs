use crate::{auth::User, util::Result};

pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, user: &User, password: &str) -> Result<Vec<u8>>;

    fn verify_password(&self, user: &User, password_hash: &[u8], password: &str) -> Result<bool>;
}
