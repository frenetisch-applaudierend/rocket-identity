use rocket::{http::Status, request::Outcome};

use crate::auth::{
    scheme::{AuthenticationSchemes, FromAuthError, MissingAuthPolicy},
    UserBuilder,
};

use super::{scheme::AuthenticationError, Authorization, Claims, Policy, Roles, UserData, UserId};

#[derive(Debug)]
pub struct User {
    id: UserId,
    username: String,
    claims: Claims,
    roles: Roles,
}

impl User {
    pub(crate) fn from_data(user_data: UserData) -> Self {
        Self {
            id: user_data
                .id
                .expect("User::from_data must be called with a user containing a UserId"),
            username: user_data.username,
            claims: user_data.claims,
            roles: user_data.roles,
        }
    }

    pub fn id(&self) -> &UserId {
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
        if self.username.is_empty() {
            return Err(UserValidationError::MissingUsername);
        }

        Ok(())
    }

    pub fn authorize<P: Policy>(&self) -> Option<Authorization<P>> {
        if P::evaluate(self) {
            Some(Authorization::new())
        } else {
            None
        }
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for &'r User {
    type Error = AuthenticationError;

    async fn from_request(req: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let outcome = req
            .local_cache_async(async move { User::create_from_request(req).await })
            .await;

        match outcome {
            Outcome::Success(user) => Outcome::Success(user),
            Outcome::Failure((status, err)) => Outcome::Failure((*status, err.clone())),
            Outcome::Forward(()) => Outcome::Forward(()),
        }
    }
}

impl User {
    async fn create_from_request(req: &rocket::Request<'_>) -> Outcome<Self, AuthenticationError> {
        use rocket::outcome::Outcome::*;

        let missing_auth_policy = MissingAuthPolicy::Fail;

        let schemes = req
            .rocket()
            .state::<AuthenticationSchemes>()
            .expect("Missing required AuthenticationSchemeCollection");

        let user_builder = UserBuilder::new();
        for scheme in schemes.iter() {
            match scheme.authenticate(req, &user_builder).await {
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
    #[error("The username is missing")]
    MissingUsername,
}
