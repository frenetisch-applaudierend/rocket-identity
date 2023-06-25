use rocket::{
    response::status::Unauthorized,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_identity::{
    auth::{hasher, UserRepository},
    config::Config,
    persistence::InMemoryRepository,
    scheme::{JwtBearer, JwtToken, JwtTokenProvider},
    RocketExt,
};

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct LoginResponse {
    username: String,
    token: JwtToken,
}

#[post("/login", format = "application/json", data = "<body>")]
async fn index(
    users: UserRepository<'_>,
    token_provider: JwtTokenProvider,
    body: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Unauthorized<()>> {
    let user = users
        .login(&body.username, &body.password)
        .await
        .map_err(|_| Unauthorized(None))?;

    let token = token_provider.generate_token(&user);

    Ok(Json(LoginResponse {
        username: body.username.to_string(),
        token,
    }))
}

#[launch]
fn rocket() -> _ {
    // Setup user repository. In a real app you'd use something
    // that actually persists users
    let hasher = hasher::default();
    let mut repository = InMemoryRepository::new();
    repository.add_user("user1", "pass1", &hasher);

    // This should be read from configuration
    let secret = "My Secret";

    rocket::build()
        .mount("/", routes![index])
        .add_identity(Config::new(repository).add_scheme(JwtBearer::new("Server", secret)))
}
