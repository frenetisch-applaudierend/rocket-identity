use rocket::{http::Status, request::Outcome};

use crate::auth::scheme::{AuthenticationSchemes, FromAuthError, MissingAuthPolicy};

use super::{scheme::AuthenticationError, Claims, Roles, UserData};

#[derive(Debug)]
pub struct User {
    id: String,
    username: String,
    claims: Claims,
    roles: Roles,
}

impl User {
    pub(crate) fn from_data(user_data: UserData) -> Self {
        Self {
            id: user_data.id,
            username: user_data.username,
            claims: user_data.claims,
            roles: user_data.roles,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn claims(&self) -> &Claims {
        &self.claims
    }

    pub fn roles(&self) -> &Roles {
        &self.roles
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

    async fn from_request(req: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        use rocket::outcome::Outcome::*;

        let missing_auth_policy = MissingAuthPolicy::Fail;

        let schemes = req
            .rocket()
            .state::<AuthenticationSchemes>()
            .expect("Missing required AuthenticationSchemeCollection");

        for scheme in schemes.iter() {
            match scheme.authenticate(req).await {
                Success(user) => {
                    user.validate()
                        .expect("Scheme created an invalid user. This is a programming error.");

                    return Success(user);
                }
                Failure(err) => return Outcome::from_err(err, missing_auth_policy),
                Forward(_) => {}
            }
        }

        match missing_auth_policy {
            MissingAuthPolicy::Fail => {
                Failure((Status::Unauthorized, AuthenticationError::Unauthenticated))
            }
            MissingAuthPolicy::Forward => Forward(()),
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
