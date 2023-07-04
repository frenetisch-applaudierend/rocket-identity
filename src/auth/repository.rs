use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

use crate::persistence::{self, UserStore};

use super::{hasher::PasswordHasher, error::LoginError, User};

pub struct UserRepository<'a> {
    repository: &'a dyn UserStore,
    hasher: &'a dyn PasswordHasher,
}

impl<'a> UserRepository<'a> {
    pub async fn login(&self, username: &str, password: &str) -> Result<User, LoginError> {
        let Some(repo_user) = self.repository.find_user_by_username(username).await.map_err(|err| LoginError::Other(err))? else {
            return Err(LoginError::UserNotFound);
        };

        let user = Self::user_from_repo(&repo_user);

        let Some(password_hash) = repo_user.password_hash else {
            return Err(LoginError::MissingPassword);
        };

        if !self
            .hasher
            .verify_password(&user, &password_hash, password)
            .map_err(LoginError::Other)?
        {
            return Err(LoginError::IncorrectPassword);
        }

        Ok(user)
    }

    fn user_from_repo(repo_user: &persistence::User) -> User {
        User {
            id: repo_user.id.clone(),
            username: repo_user.username.clone(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserRepository<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let repository = req
            .rocket()
            .state::<Box<dyn UserStore>>()
            .expect("Missing required UserRepository");

        let hasher = req
            .rocket()
            .state::<Box<dyn PasswordHasher>>()
            .expect("Missing required PasswordHasher");

        Outcome::Success(UserRepository {
            repository: &**repository,
            hasher: &**hasher,
        })
    }
}
