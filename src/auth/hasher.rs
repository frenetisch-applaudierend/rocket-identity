use crate::{util::Result, UserData};

pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, user: &UserData, password: &str) -> Result<Vec<u8>>;

    fn verify_password(
        &self,
        user: &UserData,
        password_hash: &[u8],
        password: &str,
    ) -> Result<bool>;
}
