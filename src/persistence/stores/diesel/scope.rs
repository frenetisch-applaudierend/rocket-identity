use diesel::prelude::*;

use crate::stores::impls::prelude::*;

use super::model::PersistedUser;

pub struct SqliteScope<T: 'static> {
    pub conn: rocket_sync_db_pools::Connection<T, SqliteConnection>,
}

pub struct PgScope<T: 'static> {
    pub conn: rocket_sync_db_pools::Connection<T, PgConnection>,
}

macro_rules! find_user_by_username {
    ($username:expr) => {{
        use super::schema::users;

        users::table
            .filter(users::username.eq($username))
            .select(PersistedUser::as_select())
    }};
}

#[rocket::async_trait]
impl<T> UserStoreScope for SqliteScope<T> {
    /// Find a user by their username.
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let username = username.to_string();
        let user = self
            .conn
            .run(|c| {
                find_user_by_username!(username)
                    .first(c)
                    .optional()
                    .map_err(Box::new)
            })
            .await?;

        Ok(user.map(|u| u.into()))
    }

    /// Add a user to the store.
    async fn add_user(
        &mut self,
        _user: &User,
        _password_hash: Option<&PasswordHash>,
    ) -> Result<()> {
        todo!()
    }

    /// Retrieve the password hash for a given user.
    async fn password_hash(&self, _user: &User) -> Result<Option<PasswordHash>> {
        todo!()
    }

    /// Set the password hash for a given user.
    async fn set_password_hash(
        &mut self,
        _user: &User,
        _password_hash: &PasswordHash,
    ) -> Result<()> {
        todo!()
    }
}
