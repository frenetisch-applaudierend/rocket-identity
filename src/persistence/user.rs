#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: Option<Vec<u8>>,
}
