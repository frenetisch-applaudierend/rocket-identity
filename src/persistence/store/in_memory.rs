use crate::persistence::store::prelude::*;

pub struct InMemoryUserStore {
    users: Vec<User>,
}

impl InMemoryUserStore {
    pub fn new() -> Self {
        Self { users: Vec::new() }
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

    async fn add_user(&mut self, user: &User) -> Result<UserId> {
        let mut user = user.clone();

        if (user.id).is_none() {
            user.id = Some(UserId::new_uuid());
        }

        let user_id = user.id.clone().unwrap();
        self.users.push(user);

        Ok(user_id)
    }
}

impl Default for InMemoryUserStore {
    fn default() -> Self {
        Self::new()
    }
}
