use diesel::prelude::*;

use crate::User;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
pub struct PersistedUser {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: Option<Vec<u8>>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
pub struct PasswordHashSelectable {
    pub password_hash: Option<Vec<u8>>,
}

impl From<PersistedUser> for User {
    fn from(value: PersistedUser) -> Self {
        User::with_username(value.username)
    }
}
