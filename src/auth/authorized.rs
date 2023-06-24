use std::marker::PhantomData;

use crate::{
    policy::{Any, Policy},
    scheme::AuthenticationSchemes,
};

use super::{AuthorizationError, User};

pub type Authenticated = Authorized<Any>;

pub struct Authorized<P: Policy> {
    pub user: User,
    _marker: PhantomData<P>,
}

impl<P: Policy> Authorized<P> {
    async fn get_user(
        req: &rocket::Request<'_>,
    ) -> rocket::request::Outcome<User, AuthorizationError> {
        use rocket::outcome::Outcome::*;

        let schemes = req
            .rocket()
            .state::<AuthenticationSchemes>()
            .expect("Missing required AuthenticationSchemeCollection");

        for scheme in schemes.0.iter() {
            match scheme.autenticate(req).await {
                Success(user) => return Success(user),
                Failure((status, err)) => return Failure((status, AuthorizationError::Other(err))),
                Forward(_) => (),
            }
        }

        Forward(())
    }
}

#[rocket::async_trait]
impl<'r, P: Policy> rocket::request::FromRequest<'r> for Authorized<P> {
    type Error = AuthorizationError;

    async fn from_request(
        req: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        use rocket::outcome::Outcome::*;

        let user = match Self::get_user(req).await {
            Success(user) => user,
            Failure(err) => return Failure(err),
            // Forward(_) => return Forward(()),
            Forward(_) => {
                return Failure((
                    rocket::http::Status::Unauthorized,
                    AuthorizationError::Unauthenticated,
                ))
            }
        };

        if P::evaluate(&user, req) {
            Success(Authorized {
                user,
                _marker: PhantomData::<P>,
            })
        } else {
            Failure((
                rocket::http::Status::Forbidden,
                AuthorizationError::PolicyFailed,
            ))
        }
    }
}
