use rocket::{http::Status, request::Outcome, Sentinel};

use crate::{
    auth::scheme::{AuthenticationSchemes, FromAuthError, MissingAuthPolicy},
    persistence,
};

use super::{scheme::AuthenticationError, Claims, Roles, UserData};

#[derive(Debug)]
pub struct User {
    username: String,
    claims: Claims,
    roles: Roles,
}

impl User {
    pub(crate) fn from_data(user_data: UserData) -> Self {
        Self {
            username: user_data.username,
            claims: user_data.claims,
            roles: user_data.roles,
        }
    }

    pub(crate) fn from_repo(repo_user: persistence::User) -> Self {
        Self {
            username: repo_user.username,
            claims: Claims::from_inner(repo_user.claims),
            roles: Roles::from_inner(repo_user.roles),
        }
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

            let missing_auth_policy = req
                .rocket()
                .state::<MissingAuthPolicy>()
                .expect("Missing auth policy not configured");

            let schemes = req
                .rocket()
                .state::<AuthenticationSchemes>()
                .expect("Missing required AuthenticationSchemeCollection");

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
