use rocket::{fairing::AdHoc, Orbit, Rocket};
use rocket_identity::{
    auth::{scheme::basic::Basic, User, UserData},
    persistence::store::InMemoryUserStore,
    Identity,
    Services
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index(user: &User) -> String {
    format!("Hello, {}!", user.username())
}

#[launch]
async fn rocket() -> _ {
    // Setup user repository. In a real app you'd use something
    // that actually persists users
    let user_store = InMemoryUserStore::new();

    rocket::build()
        .mount("/", routes![index])
        .attach(Identity::fairing(
            Identity::config(user_store).add_scheme(Basic::new("Hello")),
        ))
        .attach(AdHoc::on_liftoff("Setup users", |r| {
            Box::pin(setup_users(r))
        }))
}

async fn setup_users(rocket: &Rocket<Orbit>) {
    let repo = rocket.user_repository();

    repo.add_user(UserData::with_username("user1"), Some("pass1"))
        .await
        .expect("Could not add user");

    let mut admin = UserData::with_username("admin");
    admin.roles.add("admin");
    repo.add_user(admin, Some("admin"))
        .await
        .expect("Could not add user");
}
