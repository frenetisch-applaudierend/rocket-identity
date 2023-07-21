use diesel::prelude::*;

use crate::stores::impls::prelude::*;

use super::DieselConnectionProvider;
use super::model::PersistedUser;

pub struct DieselUserStoreScope<P: DieselConnectionProvider> {
    provider: P,
}

impl<P: DieselConnectionProvider> DieselUserStoreScope<P> {
    pub fn new(provider: P) -> Self {
        Self { provider }
    }
}

#[rocket::async_trait]
impl<P: DieselConnectionProvider> UserStoreScope for DieselUserStoreScope<P> {
    /// Find a user by their username.
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        use super::schema::users;

        let user = self.provider.with_connection(|c| {
            users::table
                .filter(users::username.eq(username))
                .select(PersistedUser::as_select())
                .first(c)
                .optional()
        }).await;

        todo!()
    }

    /// Add a user to the store.
    async fn add_user(&mut self, user: &User, password_hash: Option<&PasswordHash>) -> Result<()> {
        todo!()
    }

    /// Retrieve the password hash for a given user.
    async fn password_hash(&self, user: &User) -> Result<Option<PasswordHash>> {
        todo!()
    }

    /// Set the password hash for a given user.
    async fn set_password_hash(&mut self, user: &User, password_hash: &PasswordHash) -> Result<()> {
        todo!()
    }
}

impl<P: DieselConnectionProvider> core::fmt::Debug for DieselUserStoreScope<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DieselUserStoreScope").finish()
    }
}
