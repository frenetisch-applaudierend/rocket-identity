use super::{User, UserStore};
use crate::{
    auth::{hasher::PasswordHasher, UserData},
    util::Result,
};

pub struct InMemoryUserStore {
    users: Vec<User>,
}

impl InMemoryUserStore {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn add_user(
        &mut self,
        username: &str,
        password: &str,
        hasher: &dyn PasswordHasher,
        configure: impl FnOnce(&mut UserData),
    ) {
        let id = (self.users.len() + 1).to_string();

        let mut user_data = UserData {
            id,
            username: username.to_string(),
            claims: Default::default(),
            roles: Default::default(),
        };
        configure(&mut user_data);

        let password_hash = hasher
            .hash_password(&user_data, password)
            .expect("Could not hash password");

        let repo_user = User::from_data(user_data, Some(password_hash));

        self.users.push(repo_user);
    }
}

#[rocket::async_trait]
impl UserStore for InMemoryUserStore {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        for user in &self.users {
            if user.username == username {
                return Ok(Some(user.clone()));
            }
        }

        Ok(None)
    }
}

impl Default for InMemoryUserStore {
    fn default() -> Self {
        Self::new()
    }
}
