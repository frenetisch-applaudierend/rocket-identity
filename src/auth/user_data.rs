use super::{Claims, Roles, UserId};

#[derive(Debug, Clone)]
pub struct UserData {
    pub id: Option<UserId>,
    pub username: String,
    pub claims: Claims,
    pub roles: Roles,
}

impl UserData {
    pub fn with_username(username: &str) -> Self {
        Self {
            id: None,
            username: username.to_owned(),
            claims: Claims::new(),
            roles: Roles::new(),
        }
    }
}
