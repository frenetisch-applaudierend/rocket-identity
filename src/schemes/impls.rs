/// This module contains common imports for authentication schemes.
pub mod prelude {
    pub use crate::{
        hashers::PasswordHash,
        schemes::{AuthenticationError, AuthenticationScheme, Outcome},
        ClaimValue, Claims, Roles, Services, User,
    };
}
