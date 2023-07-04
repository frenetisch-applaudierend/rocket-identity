use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

use crate::persistence::UserStore;

use super::{error::LoginError, hasher::PasswordHasher, Claims, Roles, User, UserData};

pub struct UserRepository<'a> {
    store: &'a dyn UserStore,
    hasher: &'a dyn PasswordHasher,
}

impl<'a> UserRepository<'a> {
    pub async fn login(&self, username: &str, password: &str) -> Result<User, LoginError> {
        let Some(repo_user) = self.store.find_user_by_username(username).await.map_err(|err| LoginError::Other(err))? else {
            return Err(LoginError::UserNotFound);
        };

        let user_data = UserData {
            id: repo_user.id,
            username: repo_user.username,
            claims: Claims::from_inner(repo_user.claims),
            roles: Roles::from_inner(repo_user.roles),
        };

        let Some(password_hash) = repo_user.password_hash else {
            return Err(LoginError::MissingPassword);
        };

        if !self
            .hasher
            .verify_password(&user_data, &password_hash, password)
            .map_err(LoginError::Other)?
        {
            return Err(LoginError::IncorrectPassword);
        }

        Ok(self.user_from_data(user_data))
    }

    pub fn user_from_data(&self, user_data: UserData) -> User {
        User::from_data(user_data)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserRepository<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let store = req
            .rocket()
            .state::<Box<dyn UserStore>>()
            .expect("Missing required UserRepository");

        let hasher = req
            .rocket()
            .state::<Box<dyn PasswordHasher>>()
            .expect("Missing required PasswordHasher");

        Outcome::Success(UserRepository {
            store: &**store,
            hasher: &**hasher,
        })
    }
}

#[rocket::async_trait]
pub trait UserRepositoryRequestExt {
    async fn user_repository(&self) -> UserRepository;
}

#[rocket::async_trait]
impl UserRepositoryRequestExt for Request<'_> {
    async fn user_repository(&self) -> UserRepository {
        self.guard().await.expect("Missing required UserRepository")
    }
}
