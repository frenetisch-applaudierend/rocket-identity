use rocket::Request;

use crate::auth::User;

pub trait Policy {
    fn evaluate(user: &User, req: &Request) -> bool;
}
