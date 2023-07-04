use super::{Claims, Roles};

#[derive(Debug, Clone)]
pub struct UserData {
    pub id: String,
    pub username: String,
    pub claims: Claims,
    pub roles: Roles,
}
