use std::collections::{HashMap, HashSet};

use crate::auth::{ClaimValue, UserData};

#[derive(Clone, Debug)]
pub struct User<TUserId> {
    pub id: Option<TUserId>,
    pub username: String,
    pub claims: HashMap<String, ClaimValue>,
    pub roles: HashSet<String>,
    pub password_hash: Option<Vec<u8>>,
}

impl<TUserId> User<TUserId> {
    pub fn from_data(user_data: UserData<TUserId>, password_hash: Option<Vec<u8>>) -> Self {
        Self {
            id: user_data.id,
            username: user_data.username,
            claims: user_data.claims.into_inner(),
            roles: user_data.roles.into_inner(),
            password_hash,
        }
    }
}
