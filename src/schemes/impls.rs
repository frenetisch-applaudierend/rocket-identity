/// This module contains common imports for authentication schemes.
pub mod prelude {
    pub use crate::{
        schemes::{AuthenticationError, AuthenticationScheme, Outcome},
        ClaimValue, Claims, PasswordHash, Roles, Services, User,
    };
}
