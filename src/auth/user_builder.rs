use super::{User, UserData};

pub struct UserBuilder {
    _make_private: (),
}

impl UserBuilder {
    pub(crate) fn new() -> Self {
        Self { _make_private: () }
    }

    pub fn build<TUserId>(&self, data: UserData<TUserId>) -> User<TUserId> {
        User::from_data(data)
    }
}
