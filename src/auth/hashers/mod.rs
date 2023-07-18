mod identity;
mod argon2;

pub mod impls;

pub use self::argon2::*;

pub mod insecure {
    pub use super::identity::*;
}
