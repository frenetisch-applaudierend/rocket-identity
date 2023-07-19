#[derive(Debug, FromForm)]
pub struct Login<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

#[derive(Debug, FromForm)]
pub struct Registration<'r> {
    pub username: &'r str,
    pub password: &'r str,
}
