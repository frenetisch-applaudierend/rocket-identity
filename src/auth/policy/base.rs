use crate::auth::User;

/// Encodes authorization information about a User.
pub trait Policy {
    /// Evaluate the policy for the given user. Return true if the policy
    /// holds for the given user, false otherwise.
    fn evaluate(user: &User<impl Policy>) -> bool;
}
