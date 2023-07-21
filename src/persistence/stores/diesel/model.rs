use diesel::prelude::*;

use crate::User;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
pub struct PersistedUser {
    pub id: i32,
    pub username: String,
}

impl From<PersistedUser> for User {
    fn from(value: PersistedUser) -> Self {
        User::with_username(value.username)
    }
}
