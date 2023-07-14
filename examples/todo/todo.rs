use rocket_identity::auth::User;


pub fn routes() -> Vec<rocket::Route> {
    routes![list]
}

#[get("/")]
fn list(user: &User) -> String {
    format!("Hello, {}!", user.username())
}
