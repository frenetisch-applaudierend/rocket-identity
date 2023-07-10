use rocket::fairing::AdHoc;
use rocket_identity::{
    auth::{scheme::basic::Basic, Roles, User, UserData, UserRepository},
    config::Config,
    persistence::store::InMemoryUserStore,
    RocketIdentity,
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

    let rocket = rocket::build()
        .mount("/", routes![index])
        .attach(RocketIdentity::fairing(
            Config::new(user_store).add_scheme(Basic::new("Hello")),
        ))
        .attach(AdHoc::on_liftoff("Setup users", |r| {
            Box::pin(async move {
                setup_users(r.state().unwrap()).await;
            })
        }));

    rocket
}

async fn setup_users(user_repository: &UserRepository) {
    // Add users to the user repository
    user_repository
        .add_user(
            UserData {
                id: None,
                username: "user1".to_string(),
                claims: Default::default(),
                roles: Default::default(),
            },
            Some("pass1"),
        )
        .await
        .expect("Failed to add user");

    user_repository
        .add_user(
            UserData {
                id: None,
                username: "admin".to_string(),
                claims: Default::default(),
                roles: Roles::from(vec!["admin"]),
            },
            Some("admin"),
        )
        .await
        .expect("Failed to add user");
}
