pub mod prelude {
    pub use crate::{
        hashers::PasswordHash,
        stores::{AddUserError, FindUserError, PasswordHashError, UserStore, UserStoreScope},
        util::BoxableError,
        User,
    };
}
