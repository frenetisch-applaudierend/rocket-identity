use std::num::NonZeroU32;

use ring::pbkdf2;

use crate::auth::User;

use super::PasswordHasher;

pub struct Pbkdf2PasswordHasher {
    alg: pbkdf2::Algorithm,
    iterations: NonZeroU32,
    salt_seed: Vec<u8>,
    hash_len: usize,
}

impl Pbkdf2PasswordHasher {
    pub fn new(
        alg: pbkdf2::Algorithm,
        iterations: u32,
        salt_seed: Vec<u8>,
        hash_len: usize,
    ) -> Self {
        Self {
            alg,
            iterations: NonZeroU32::new(iterations).expect("iterations must be a positive integer"),
            salt_seed,
            hash_len,
        }
    }

    pub fn default(salt_seed: Vec<u8>) -> Self {
        Self::new(
            pbkdf2::PBKDF2_HMAC_SHA256,
            100_000,
            salt_seed,
            ring::digest::SHA256_OUTPUT_LEN,
        )
    }

    fn generate_salt(&self, user: &User) -> Vec<u8> {
        let mut salt = Vec::with_capacity(self.salt_seed.len() + user.username.as_bytes().len());
        salt.extend(self.salt_seed.as_slice());
        salt.extend(user.username.as_bytes());
        salt
    }
}

impl PasswordHasher for Pbkdf2PasswordHasher {
    fn hash_password(&self, user: &User, password: &str) -> Vec<u8> {
        let salt = self.generate_salt(user);

        let mut hash = vec![0; self.hash_len];

        pbkdf2::derive(
            self.alg,
            self.iterations,
            &salt,
            password.as_bytes(),
            &mut hash,
        );

        hash
    }

    fn verify_hash(&self, user: &User, password_hash: &[u8], password: &str) -> bool {
        let salt = self.generate_salt(user);

        let result = pbkdf2::verify(
            self.alg,
            self.iterations,
            &salt,
            password.as_bytes(),
            password_hash,
        );

        result.is_ok()
    }
}

#[cfg(test)]
mod test {
    use crate::auth::{hasher::PasswordHasher, User};

    use super::Pbkdf2PasswordHasher;

    #[test]
    fn test_roundtrip() {
        let salt_seed = vec![0, 1, 2, 3, 4, 5];
        let hasher = Pbkdf2PasswordHasher::default(salt_seed);

        let user = User {
            id: "1".to_owned(),
            username: "user1".to_owned(),
        };

        let password = "my super secure password";

        let hash = hasher.hash_password(&user, password);
        let verified = hasher.verify_hash(&user, &hash, password);

        assert!(verified);
    }

    #[test]
    fn test_invalid() {
        let salt_seed = vec![0, 1, 2, 3, 4, 5];
        let hasher = Pbkdf2PasswordHasher::default(salt_seed);

        let user = User {
            id: "1".to_owned(),
            username: "user1".to_owned(),
        };

        let password = "my super secure password";
        let wrong_password = "some other password";

        let hash = hasher.hash_password(&user, password);
        let verified = hasher.verify_hash(&user, &hash, wrong_password);

        assert!(!verified);
    }
}
