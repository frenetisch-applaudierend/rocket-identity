use crate::auth::User;

pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, user: &User, password: &str) -> Vec<u8>;

    fn verify_hash(&self, user: &User, password_hash: &[u8], password: &str) -> bool {
        let checked_hash = self.hash_password(user, password);
        ring::constant_time::verify_slices_are_equal(password_hash, &checked_hash).is_ok()
    }
}
