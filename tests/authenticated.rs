use rocket::{
    catch, catchers,
    fairing::AdHoc,
    get,
    http::{Header, Status},
    local::blocking::Client,
    routes, Build, Request, Rocket,
};
use rocket_identity::{
    auth::{scheme::basic::Basic, User, UserData, UserRepositoryAccessor},
    config::Config,
    persistence::store::InMemoryUserStore,
    RocketIdentity,
};

#[get("/authenticated")]
fn handler(user: &User<u32>) -> &str {
    user.username()
}

#[catch(401)]
fn catch_unauthorized(_req: &Request) -> &'static str {
    "unauthorized"
}

fn setup() -> Rocket<Build> {
    let config = Config::new(InMemoryUserStore::new()).add_scheme(Basic::new("Server"));

    rocket::build()
        .mount("/", routes![handler])
        .register("/", catchers![catch_unauthorized])
        .attach(RocketIdentity::fairing(config))
        .attach(AdHoc::on_liftoff("User setup", |r| Box::pin(initialize(r))))
}

async fn initialize(rocket: &Rocket<rocket::Orbit>) {
    let repo = rocket.user_repository::<u32>();

    repo.add_user(UserData::with_username("user1"), Some("pass1"))
        .await
        .expect("Could not add user");
}

#[test]
fn request_with_valid_credentials_succeeds() {
    let rocket = setup();
    let client = Client::tracked(rocket).expect("Failed to acquire Client");

    let mut req = client.get("/authenticated");
    req.add_header(Header::new("Authorization", "Basic dXNlcjE6cGFzczE=")); // user1:pass1
    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);
    assert!(!res.headers().contains("WWW-Authenticate"));
    assert_eq!(res.into_string().expect("Unexpected body"), "user1");
}

#[test]
fn request_with_invalid_credentials_fails() {
    let rocket = setup();
    let client = Client::tracked(rocket).expect("Failed to acquire Client");

    let mut req = client.get("/authenticated");
    req.add_header(Header::new("Authorization", "Basic dXNlcjE6d3JvbmdwYXNz")); // user1:wrongpass
    let res = req.dispatch();

    assert_eq!(res.status(), Status::Unauthorized);
    assert_eq!(
        res.headers().get("WWW-Authenticate").collect::<Vec<_>>(),
        vec![r#"Basic realm="Server", charset="UTF-8""#]
    );
    assert_ne!(res.into_string().expect("Unexpected body"), "user1");
}

#[test]
fn request_without_credentials_fails() {
    let rocket = setup();
    let client = Client::tracked(rocket).expect("Failed to acquire Client");

    let req = client.get("/authenticated");

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Unauthorized);
    assert_eq!(
        res.headers().get("WWW-Authenticate").collect::<Vec<_>>(),
        vec![r#"Basic realm="Server", charset="UTF-8""#]
    );
    assert_ne!(res.into_string().expect("Unexpected body"), "user1");
}
