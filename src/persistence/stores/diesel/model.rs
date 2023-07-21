use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
pub struct PersistedUser {
    pub id: i32,
    pub username: String,
}
