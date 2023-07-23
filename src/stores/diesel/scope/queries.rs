macro_rules! find_user_by_username {
    ($username:expr) => {{
        use crate::stores::diesel::schema::users;
        use crate::stores::diesel::model::PersistedUser;

        users::table
            .filter(users::username.eq($username))
            .select(PersistedUser::as_select())
    }};
}

macro_rules! add_user {
    ($user:expr) => {{
        use crate::stores::diesel::schema::users;

        diesel::insert_into(users::table)
            .values($user)
    }};
}

macro_rules! get_password_hash {
    ($username:expr) => {{
        use crate::stores::diesel::schema::users;
        use crate::stores::diesel::model::PasswordHashSelectable;

        users::table
            .filter(users::username.eq($username))
            .select(PasswordHashSelectable::as_select())
    }};
}

macro_rules! set_password_hash {
    ($username:expr, $hash:expr) => {{
        use crate::stores::diesel::schema::users;

        diesel::update(users::table)
            .filter(users::username.eq($username))
            .set(users::password_hash.eq($hash))
    }};
}

pub(crate) use find_user_by_username;
pub(crate) use add_user;
pub(crate) use get_password_hash;
pub(crate) use set_password_hash;