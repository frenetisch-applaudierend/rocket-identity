use rocket::{get, http::Status, local::blocking::Client, routes};

#[get("/unauthenticated")]
fn handler() -> &'static str {
    "ok"
}

#[test]
fn request_without_credentials_succeeds() {
    let rocket = rocket::build().mount("/", routes![handler]);
    let client = Client::tracked(rocket).expect("Failed to acquire Client");

    let req = client.get("/unauthenticated");

    let res = req.dispatch();

    assert_eq!(res.status(), Status::Ok);
    assert!(!res.headers().contains("WWW-Authenticate"));
    assert_eq!(res.into_string().expect("Unexpected body"), "ok");
}
