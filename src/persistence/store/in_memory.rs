use crate::persistence::store::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct InMemoryUserStore<TGen: IdGenerator> {
    users: Vec<User<TGen::UserId>>,
    id_gen: TGen,
}

pub trait IdGenerator: Send + Sync {
    type UserId: 'static + Send + Sync + Clone;

    fn next(&mut self) -> Self::UserId;
}

impl<TGen: IdGenerator> InMemoryUserStore<TGen> {
    pub fn new(id_gen: TGen) -> Self {
        Self {
            users: Vec::new(),
            id_gen,
        }
    }
}

#[rocket::async_trait]
impl<TGen: IdGenerator> UserStore<TGen::UserId> for InMemoryUserStore<TGen> {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User<TGen::UserId>>> {
        for user in &self.users {
            if user.username == username {
                return Ok(Some(user.clone()));
            }
        }

        Ok(None)
    }

    async fn add_user(&mut self, user: &mut User<TGen::UserId>) -> Result<()> {
        user.id = Some(self.id_gen.next());
        self.users.push(user.clone());

        Ok(())
    }
}

pub struct U32Generator {
    cur: u32,
}

impl U32Generator {
    pub fn new() -> Self {
        U32Generator { cur: 0 }
    }
}

impl IdGenerator for U32Generator {
    type UserId = u32;

    fn next(&mut self) -> Self::UserId {
        self.cur += 1;
        self.cur
    }
}

pub struct StringGenerator;

impl IdGenerator for StringGenerator {
    type UserId = String;

    fn next(&mut self) -> Self::UserId {
        uuid::Uuid::new_v4().to_string()
    }
}
