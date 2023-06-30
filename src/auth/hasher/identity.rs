use crate::auth::User;

use super::PasswordHasher;

pub struct IdentityPasswordHasher;

impl PasswordHasher for IdentityPasswordHasher {
    fn hash_password(&self, _user: &User, password: &str) -> Vec<u8> {
        password.bytes().collect()
    }
}