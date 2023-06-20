pub struct Any;

impl super::Policy for Any {
    fn evaluate(_user: &crate::auth::User, _req: &rocket::Request) -> bool {
        true
    }
}
