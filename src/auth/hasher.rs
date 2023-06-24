use super::User;

pub trait PasswordHasher: Send + Sync {
    fn hash_length(&self) -> Option<usize>;

    fn hash_password(&self, user: &User, password: &str) -> Vec<u8>;

    fn verify_hash(&self, user: &User, password_hash: &[u8], password: &str) -> bool {
        if self
            .hash_length()
            .is_some_and(|len| password_hash.len() != len)
        {
            return false;
        }

        let checked_hash = self.hash_password(user, password);

        ring::constant_time::verify_slices_are_equal(password_hash, &checked_hash).is_ok()
    }
}

pub struct IdentityPasswordHasher;

impl PasswordHasher for IdentityPasswordHasher {
    fn hash_length(&self) -> Option<usize> {
        None
    }

    fn hash_password(&self, _user: &User, password: &str) -> Vec<u8> {
        password.bytes().collect()
    }
}
