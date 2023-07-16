//! This module contains common imports for authentication schemes.

pub use super::AuthenticationError;
pub use super::AuthenticationScheme;
pub use super::Outcome;

pub use crate::auth::UserRepositoryAccessor;
pub use crate::auth::{ClaimValue, Claims, Roles, UserBuilder, UserData};
