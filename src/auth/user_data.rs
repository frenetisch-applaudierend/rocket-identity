use super::{Claims, Roles, UserId};

#[derive(Debug, Clone)]
pub struct UserData {
    pub id: Option<UserId>,
    pub username: String,
    pub claims: Claims,
    pub roles: Roles,
}
