use crate::hashers::impls::prelude::*;

#[derive(Debug)]
pub struct IdentityPasswordHasher;

impl PasswordHasher for IdentityPasswordHasher {
    fn hash_password(&self, _user: &User, password: &str) -> Result<PasswordHash> {
        Ok(password.as_bytes().into())
    }

    fn verify_password(
        &self,
        _user: &User,
        password_hash: &PasswordHash,
        password: &str,
    ) -> Result<bool> {
        Ok(password.as_bytes() == password_hash.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use crate::{hashers::PasswordHasher, User};

    #[test]
    fn test_roundtrip() {
        let hasher = super::IdentityPasswordHasher;

        let user = User::with_username("user1");

        let password = "my super secure password";

        let hash = hasher
            .hash_password(&user, password)
            .expect("Always succeeds");
        let verified = hasher
            .verify_password(&user, &hash, password)
            .expect("Always succeeds");

        assert!(verified);
    }

    #[test]
    fn test_invalid() {
        let hasher = super::IdentityPasswordHasher;

        let user = User::with_username("user1");

        let password = "my super secure password";
        let wrong_password = "wrong password";

        let hash = hasher
            .hash_password(&user, password)
            .expect("Always succeeds");

        let verified = hasher
            .verify_password(&user, &hash, wrong_password)
            .expect("Always succeeds");

        assert!(!verified);
    }
}
