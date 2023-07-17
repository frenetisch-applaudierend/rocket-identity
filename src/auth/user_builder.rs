use super::{User, UserData};

pub struct UserBuilder {
    _make_private: (),
}

impl UserBuilder {
    pub(crate) fn new() -> Self {
        Self { _make_private: () }
    }

    pub fn build(&self, data: UserData) -> User {
        User::from_data(data)
    }
}
