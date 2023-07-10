use jsonwebtoken::{DecodingKey, EncodingKey};
use rocket::{
    response::status::Unauthorized,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_identity::{
    auth::{
        hasher,
        scheme::jwt::{JwtBearer, JwtConfig, JwtToken, JwtTokenProvider},
        Authorization, Policy, User, UserRepository,
    },
    config::{Config, RocketIdentityInitializer},
    persistence::InMemoryUserStore,
    RocketIdentity,
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

struct Admin;

impl Policy for Admin {
    fn evaluate(user: &User) -> bool {
        user.roles().contains("admin")
    }
}

#[post("/login", format = "application/json", data = "<body>")]
async fn login(
    users: &UserRepository,
    token_provider: JwtTokenProvider<'_>,
    body: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Unauthorized<()>> {
    let user = users
        .login(&body.username, &body.password)
        .await
        .map_err(|_| Unauthorized(None))?;

    let token = token_provider
        .generate_token(&user)
        .map_err(|_| Unauthorized(None))?;

    Ok(Json(LoginResponse {
        username: body.username.to_string(),
        token,
    }))
}

#[get("/")]
fn index(user: &User) -> String {
    format!("Hello, {}!", user.username())
}

#[get("/admin")]
fn admin(user: &User, _admin: Authorization<Admin>) -> String {
    format!("Hello, Admin {}!", user.username())
}

#[launch]
fn rocket() -> _ {
    // Create a password hasher. In a real app you'd use hasher::default(<salt>) or
    // another hasher that is secure
    let hasher = hasher::insecure::IdentityPasswordHasher;

    // Setup user repository. In a real app you'd use something
    // that actually persists users
    let mut user_store = InMemoryUserStore::new();
    user_store.add_user("user1", "pass1", &hasher, |_| {});
    user_store.add_user("admin", "admin", &hasher, |u| {
        u.roles.add("admin");
    });

    // This should be read from configuration
    let secret = b"My Secret";
    let jwt_config = JwtConfig {
        encoding_key: EncodingKey::from_secret(secret),
        deconding_key: DecodingKey::from_secret(secret),
    };

    let config = Config::new(user_store)
        .add_scheme(JwtBearer::new(jwt_config));

    rocket::build()
        .mount("/", routes![login, index, admin])
        .attach(RocketIdentity::fairing(config))
}

async fn setup_users(user_repository: &UserRepository) {

}