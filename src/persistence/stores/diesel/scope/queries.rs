macro_rules! find_user_by_username {
    ($username:expr) => {{
        use crate::stores::diesel::schema::users;
        use crate::stores::diesel::model::PersistedUser;

        users::table
            .filter(users::username.eq($username))
            .select(PersistedUser::as_select())
    }};
}

pub(crate) use find_user_by_username;