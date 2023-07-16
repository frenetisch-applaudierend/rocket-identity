use crate::{auth::UserData, util::Result};

pub trait PasswordHasher<TUserId>: Send + Sync {
    fn hash_password(&self, user: &UserData<TUserId>, password: &str) -> Result<Vec<u8>>;

    fn verify_password(
        &self,
        user: &UserData<TUserId>,
        password_hash: &[u8],
        password: &str,
    ) -> Result<bool>;
}
