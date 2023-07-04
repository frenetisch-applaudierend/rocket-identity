use crate::auth::scheme::{AuthenticationSchemes, MissingAuthPolicy};

use super::{scheme::AuthenticationError, Claims, Roles};

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub claims: Claims,
    pub roles: Roles,

    _make_private: (),
}

impl User {
    /// Create a new user without any values set.
    pub(crate) fn empty() -> Self {
        Self {
            id: String::new(),
            username: String::new(),
            claims: Claims::new(),
            roles: Roles::new(),

            _make_private: (),
        }
    }

    pub fn validate(&self) -> Result<(), UserValidationError> {
        if self.id.is_empty() {
            return Err(UserValidationError::MissingId);
        }

        if self.username.is_empty() {
            return Err(UserValidationError::MissingUsername);
        }

        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for User {
    type Error = AuthenticationError;

    async fn from_request(
        req: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        use rocket::outcome::Outcome::*;

        let missing_auth_policy = MissingAuthPolicy::Fail;

        let schemes = req
            .rocket()
            .state::<AuthenticationSchemes>()
            .expect("Missing required AuthenticationSchemeCollection");

        let mut user = User::empty();
        for scheme in schemes.iter() {
            match scheme.autenticate(&mut user, req).await {
                Success(_) => return Success(user),
                Failure(failure) => return (failure, missing_auth_policy).into(),
                Forward(_) => (),
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserValidationError {
    #[error("The user ID is missing")]
    MissingId,

    #[error("The username is missing")]
    MissingUsername,
}
