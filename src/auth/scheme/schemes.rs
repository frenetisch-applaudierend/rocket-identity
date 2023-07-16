use rocket::Request;

use crate::auth::UserBuilder;

use super::{AuthenticationScheme, Outcome};

/// A collection of authentication schemes.
pub(crate) struct AuthenticationSchemes<TUserId>(Vec<Box<dyn AuthenticationScheme<TUserId>>>);

impl<TUserId> AuthenticationSchemes<TUserId> {
    /// Create a new collection of authentication schemes.
    pub fn new(schemes: Vec<Box<dyn AuthenticationScheme<TUserId>>>) -> Self {
        Self(schemes)
    }

    /// Check if the authentication scheme collection is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Create an iterator over the authentication schemes.
    pub fn iter(&self) -> impl Iterator<Item = &dyn AuthenticationScheme<TUserId>> {
        self.0.iter().map(|b| &**b)
    }

    /// Create a mutable iterator over the authentication schemes.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut dyn AuthenticationScheme<TUserId>> {
        self.0
            .iter_mut()
            .map(|b| -> &mut dyn AuthenticationScheme<TUserId> { &mut **b })
    }

    /// Try to authenticate a user using the authentication schemes in order.
    pub async fn authenticate(&self, req: &Request<'_>) -> Outcome<TUserId> {
        let user_builder = UserBuilder::new();
        for scheme in self.iter() {
            match scheme.authenticate(req, &user_builder).await {
                Outcome::Success(user) => {
                    user.validate()
                        .expect("Scheme created an invalid user. This is a programming error.");

                    return Outcome::Success(user);
                }
                Outcome::Failure(err) => return Outcome::Failure(err),
                Outcome::Forward(_) => {}
            }
        }

        Outcome::Forward(())
    }
}
