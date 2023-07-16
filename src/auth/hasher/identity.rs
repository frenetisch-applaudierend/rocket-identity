use crate::{auth::UserData, util::Result};

use super::PasswordHasher;

pub struct IdentityPasswordHasher;

impl<TUserId> PasswordHasher<TUserId> for IdentityPasswordHasher {
    fn hash_password(&self, _user: &UserData<TUserId>, password: &str) -> Result<Vec<u8>> {
        Ok(password.bytes().collect())
    }

    fn verify_password(
        &self,
        _user: &UserData<TUserId>,
        password_hash: &[u8],
        password: &str,
    ) -> Result<bool> {
        Ok(password.as_bytes() == password_hash)
    }
}

#[cfg(test)]
mod test {
    use crate::auth::{hasher::PasswordHasher, UserData};

    #[test]
    fn test_roundtrip() {
        let hasher = super::IdentityPasswordHasher;

        let user = UserData::<u32> {
            id: None,
            username: "user1".to_owned(),
            claims: Default::default(),
            roles: Default::default(),
        };

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

        let user = UserData::<u32> {
            id: None,
            username: "user1".to_owned(),
            claims: Default::default(),
            roles: Default::default(),
        };

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
