use super::{Claims, Roles};

#[derive(Debug, Clone)]
pub struct UserData<TUserId> {
    pub id: Option<TUserId>,
    pub username: String,
    pub claims: Claims,
    pub roles: Roles,
}

impl<TUserId> UserData<TUserId> {
    pub fn with_username(username: &str) -> Self {
        Self {
            id: None,
            username: username.to_owned(),
            claims: Claims::new(),
            roles: Roles::new(),
        }
    }
}
