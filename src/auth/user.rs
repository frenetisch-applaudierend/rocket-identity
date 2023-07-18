use rocket::{http::Status, request::Outcome, Sentinel};

use crate::{
    config::MissingAuthPolicy, AuthenticationError, AuthenticationSchemes, Claims, FromAuthError,
    InternalServices, Roles, Services,
};

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub claims: Claims,
    pub roles: Roles,
}

impl User {
    pub fn with_username(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            claims: Claims::new(),
            roles: Roles::new(),
        }
    }
    
    pub fn validate(&self) -> Result<(), UserValidationError> {
        if self.username.is_empty() {
            return Err(UserValidationError::MissingUsername);
        }

        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for &'r User {
    type Error = AuthenticationError;

    async fn from_request(req: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let outcome = req
            .local_cache_async(async move { create_from_request(req).await })
            .await;

        return match outcome {
            Outcome::Success(user) => Outcome::Success(user),
            Outcome::Failure((status, err)) => Outcome::Failure((*status, err.clone())),
            Outcome::Forward(()) => Outcome::Forward(()),
        };

        async fn create_from_request(
            req: &rocket::Request<'_>,
        ) -> Outcome<User, AuthenticationError> {
            use rocket::outcome::Outcome::*;

            let missing_auth_policy = req.missing_auth_policy();
            let schemes = req.authentication_schemes();

            match schemes.authenticate(req).await {
                Success(user) => Success(user),
                Failure(err) => return Outcome::from_err(err, *missing_auth_policy),
                Forward(_) => match missing_auth_policy {
                    MissingAuthPolicy::Fail => {
                        Failure((Status::Unauthorized, AuthenticationError::Unauthenticated))
                    }
                    MissingAuthPolicy::Forward => Forward(()),
                },
            }
        }
    }
}

impl Sentinel for &User {
    fn abort(rocket: &rocket::Rocket<rocket::Ignite>) -> bool {
        let err = "Authentication schemes are not configured. Attach Identity::fairing() on your rocket instance and make sure you have at least one scheme added using add_scheme().";
        let Some(auth_schemes) = rocket.state::<AuthenticationSchemes>() else {
            log::error!("{}", err);
            return true;
        };

        if auth_schemes.is_empty() {
            log::error!("{}", err);
            return true;
        }

        false
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserValidationError {
    #[error("The username is missing")]
    MissingUsername,
}
