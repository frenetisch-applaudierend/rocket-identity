use diesel::prelude::*;

use crate::stores::{diesel::model::NewUser, impls::prelude::*};

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
    async fn add_user(&mut self, user: &User, password_hash: Option<&PasswordHash>) -> Result<()> {
        let new_user = NewUser {
            username: user.username.clone(),
            password_hash: password_hash.map(|h| h.clone().into_inner()),
        };

        self.conn
            .run(|c| queries::add_user!(new_user).execute(c).map_err(Box::new))
            .await?;

        Ok(())
    }

    /// Retrieve the password hash for a given user.
    async fn password_hash(&self, user: &User) -> Result<Option<PasswordHash>> {
        let username = user.username.to_string();
        let hash = self
            .conn
            .run(|c| {
                queries::get_password_hash!(username)
                    .first(c)
                    .optional()
                    .map_err(Box::new)
            })
            .await?;

        let hash = hash.and_then(|h| h.password_hash).map(PasswordHash::from);

        Ok(hash)
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
