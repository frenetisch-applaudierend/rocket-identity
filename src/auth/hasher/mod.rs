mod base;
mod identity;

// #[cfg(argon2)]
mod argon2;

pub use base::*;

// #[cfg(argon2)]
pub use self::argon2::*;

pub mod insecure {
    pub use super::identity::*;
}
