use rocket_identity::{
    auth::Authenticated, config::Config, persistence::InMemoryRepository, scheme::Basic, RocketExt,
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index(auth: Authenticated) -> String {
    format!("Hello, {}!", auth.user.username)
}

#[launch]
fn rocket() -> _ {
    // Setup user repository. In a real app you'd use something
    // that actually persists users
    let hasher = rocket_identity::auth::hasher::default();
    let mut repository = InMemoryRepository::new();
    repository.add_user("user1", "pass1", &hasher);

    rocket::build()
        .mount("/", routes![index])
        .add_identity(Config::new(repository).add_scheme(Basic::new("Hello")))
}
