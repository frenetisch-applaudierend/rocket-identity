use rocket_identity::{
    auth::scheme::basic::Basic, config::Config, persistence::store::InMemoryUserStore,
    RocketIdentity,
};

#[macro_use]
extern crate rocket;

mod users;

#[launch]
fn rocket() -> _ {
    let user_store = InMemoryUserStore::new();
    let identity_config = Config::new(user_store).add_scheme(Basic::new("Server"));

    rocket::build()
        .attach(RocketIdentity::fairing(identity_config))
        .mount("/users", users::routes())
}
