use std::marker::PhantomData;

use rocket::{request::{FromRequest, Outcome}, http::Status};

use super::{scheme::AuthenticationError, Policy, User};

pub struct Authorization<P: Policy> {
    _policy: PhantomData<P>,
}

impl<P: Policy> Authorization<P> {
    pub(crate) fn new() -> Self {
        Self {
            _policy: PhantomData,
        }
    }
}

#[rocket::async_trait]
impl<'r, P: Policy> FromRequest<'r> for Authorization<P> {
    type Error = AuthorizationError;

    async fn from_request(req: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let user = match <&User>::from_request(req).await {
            Outcome::Success(user) => user,
            Outcome::Failure((status, err)) => {
                return Outcome::Failure((status, AuthorizationError::Unauthenticated(err)))
            }
            Outcome::Forward(()) => return Outcome::Forward(()),
        };

        let Some(authorization) = user.authorize::<P>() else {
            return Outcome::Failure((Status::Forbidden, AuthorizationError::Unauthorized));
        };

        Outcome::Success(authorization)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthorizationError {
    #[error("The user could not authenticated")]
    Unauthenticated(AuthenticationError),

    #[error("The user could not be authorized against the specified policy")]
    Unauthorized,
}
