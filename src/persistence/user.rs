use std::collections::{HashMap, HashSet};

use crate::{ClaimValue, UserData};

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    pub claims: HashMap<String, ClaimValue>,
    pub roles: HashSet<String>,
    pub password_hash: Option<Vec<u8>>,
}

impl User {
    pub fn from_data(user_data: UserData, password_hash: Option<Vec<u8>>) -> Self {
        Self {
            username: user_data.username,
            claims: user_data.claims.into_inner(),
            roles: user_data.roles.into_inner(),
            password_hash,
        }
    }
}
