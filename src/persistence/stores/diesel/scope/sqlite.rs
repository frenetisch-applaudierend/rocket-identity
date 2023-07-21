use diesel::prelude::*;

use crate::stores::impls::prelude::*;

use super::queries;

pub struct SqliteScope<T: 'static> {
    pub conn: rocket_sync_db_pools::Connection<T, SqliteConnection>,
}

#[rocket::async_trait]
impl<T> UserStoreScope for SqliteScope<T> {
    /// Find a user by their username.
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let username = username.to_string();
        let user = self
            .conn
            .run(|c| {
                queries::find_user_by_username!(username)
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
