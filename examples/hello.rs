use rocket::{fairing::AdHoc, Orbit, Rocket};
use rocket_identity::{
    schemes::basic::Basic,
    stores::in_memory::InMemoryUserStore,
    {Identity, Services, User},
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index(user: &User) -> String {
    format!("Hello, {}!", user.username)
}

#[launch]
async fn rocket() -> _ {
    // Setup user repository. In a real app you'd use something
    // that actually persists users
    let user_store = InMemoryUserStore::new();

    rocket::build()
        .mount("/", routes![index])
        .attach(Identity::fairing(
            Identity::config()
                .with_user_store(user_store)
                .add_scheme(Basic::new("Hello"))
                .build(),
        ))
        .attach(AdHoc::on_liftoff("Setup users", |r| {
            Box::pin(setup_users(r))
        }))
}

async fn setup_users(rocket: &Rocket<Orbit>) {
    let users = rocket.user_repository().await;

    users.add_user(&User::with_username("user1"), Some("pass1"))
        .await
        .expect("Could not add user");

    let admin = &mut User::with_username("admin");
    admin.roles.add("admin");
    users.add_user(admin, Some("admin"))
        .await
        .expect("Could not add user");
}
