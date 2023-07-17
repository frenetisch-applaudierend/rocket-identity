use super::{Claims, Roles};

#[derive(Debug, Clone)]
pub struct UserData {
    pub username: String,
    pub claims: Claims,
    pub roles: Roles,
}

impl UserData {
    pub fn with_username(username: &str) -> Self {
        Self {
            username: username.to_owned(),
            claims: Claims::new(),
            roles: Roles::new(),
        }
    }
}
