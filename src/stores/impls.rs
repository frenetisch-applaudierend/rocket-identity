pub mod prelude {
    pub use crate::{
        hashers::PasswordHash,
        stores::{UserStore, UserStoreScope},
        util::Result,
        User,
    };
}
