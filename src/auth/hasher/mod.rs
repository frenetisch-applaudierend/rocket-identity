mod base;
mod identity;
mod pbkdf2;

pub use base::*;
pub use pbkdf2::*;

pub fn default(salt_seed: Vec<u8>) -> impl PasswordHasher {
    Pbkdf2PasswordHasher::default(salt_seed)
}

pub mod insecure {
    pub use super::identity::*;
}
