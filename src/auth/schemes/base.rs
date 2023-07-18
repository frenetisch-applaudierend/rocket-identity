use rocket::http::Status;
use user_builder::UserBuilder;

use crate::auth::{user_builder, User};

/// Encodes information about a way to authenticate a User.
#[rocket::async_trait]
pub trait AuthenticationScheme: Send + Sync + core::fmt::Debug {
    /// The name of this authentication scheme.
    fn name(&self) -> String;

    /// Setup the authentication scheme. This is called once when the server starts and gives
    /// the scheme a chance to do any necessary setup like registering state.
    fn setup(&mut self, rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
        rocket
    }

    /// Try to authenticate a user. If the user is successfully authenticated, mutate the user with the correct values and return Success.
    /// If authentication was applicable but failed, return Failure with an appropriate HTTP status code and an error describing the problem.
    /// If authentication was not applicable, return Forward.
    async fn authenticate(
        &self,
        req: &rocket::Request,
        user_builder: &UserBuilder,
    ) -> Outcome;

    /// Add challenge information for the client to the response.
    /// Usually by adding a WWW-Authenticate header for this authentication scheme.
    async fn challenge(&self, res: &mut rocket::Response);
}

/// The outcome of an authentication attempt. Success means that the attempt was
/// successful. Failure means that the scheme was applicable but authentication failed
/// e.g. because of invalid credentials. Forward means that the scheme was not applicable
/// and a different scheme should be tried.
///
/// When Failure is returned, a HTTP status code and an error must be specified.
pub type Outcome = rocket::outcome::Outcome<User, AuthenticationError, ()>;

pub(crate) trait FromAuthError {
    fn from_err(err: AuthenticationError, policy: MissingAuthPolicy) -> Self;
}

/// Create an outcome from a given AuthenticationError and MissingAuthPolicy.
impl<T> FromAuthError for rocket::request::Outcome<T, AuthenticationError> {
    fn from_err(err: AuthenticationError, policy: MissingAuthPolicy) -> Self {
        let status = match err {
            AuthenticationError::Unauthenticated => Status::Unauthorized,
            AuthenticationError::InvalidParams => Status::BadRequest,
            AuthenticationError::Other => Status::InternalServerError,
        };

        let should_forward = match policy {
            MissingAuthPolicy::Fail => false,
            MissingAuthPolicy::Forward => true,
        };

        if should_forward && status == Status::Unauthorized {
            rocket::request::Outcome::Forward(())
        } else {
            rocket::request::Outcome::Failure((status, err))
        }
    }
}

/// An error that can happen during authentication.
#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum AuthenticationError {
    #[error("The user is not authenticated")]
    Unauthenticated,

    #[error("The supplied authentication parameters are not valid")]
    InvalidParams,

    #[error("Some other error happened")]
    Other,
}

#[derive(Debug, Clone, Copy)]
pub enum MissingAuthPolicy {
    Fail,
    Forward,
}
