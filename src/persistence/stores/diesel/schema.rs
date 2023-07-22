diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Nullable<Binary>,
    }
}