use crate::auth::{Policy, User};

/// A policy that holds for all users.
pub struct Any;

impl Policy for Any {
    fn evaluate(_user: &User) -> bool {
        true
    }
}
