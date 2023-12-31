use jsonwebtoken::{DecodingKey, EncodingKey};
use rocket::{
    fairing::AdHoc,
    response::status::Unauthorized,
    serde::{json::Json, Deserialize, Serialize},
    Orbit, Rocket,
};
use rocket_identity::{
    schemes::jwt::{JwtBearer, JwtConfig, JwtToken, JwtTokenProvider},
    stores::memory::MemoryStore,
    {Identity, Services, User, UserRepository},
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

// struct Admin;

// impl Policy for Admin {
//     fn evaluate(user: &User) -> bool {
//         user.roles().contains("admin")
//     }
// }

#[post("/login", format = "application/json", data = "<body>")]
async fn login(
    users: &UserRepository,
    token_provider: JwtTokenProvider<'_>,
    body: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Unauthorized<()>> {
    let user = users
        .authenticate(&body.username, &body.password)
        .await
        .map_err(|_| Unauthorized(None))?;

    let token = token_provider
        .create_token(&user)
        .map_err(|_| Unauthorized(None))?;

    Ok(Json(LoginResponse {
        username: body.username.to_string(),
        token,
    }))
}

#[get("/")]
fn index(user: &User) -> String {
    format!("Hello, {}!", user.username)
}

#[get("/admin")]
fn admin(user: &User /*_admin: Authorization<Admin>*/) -> String {
    format!("Hello, Admin {}!", user.username)
}

#[launch]
fn rocket() -> _ {
    // Setup user backing store. In a real app you'd use something
    // that actually persists users
    let user_store = MemoryStore::new();

    // This should be read from configuration
    let secret = b"My Secret";
    let jwt_config = JwtConfig {
        encoding_key: EncodingKey::from_secret(secret),
        deconding_key: DecodingKey::from_secret(secret),
    };

    let config = Identity::config()
        .with_user_store(user_store)
        .add_scheme(JwtBearer::new(jwt_config))
        .build();

    rocket::build()
        .mount("/", routes![login, index, admin])
        .attach(Identity::fairing(config))
        .attach(AdHoc::on_liftoff("User setup", |r| {
            Box::pin(setup_users(r))
        }))
}

async fn setup_users(rocket: &Rocket<Orbit>) {
    let users = rocket.user_repository().await;

    users
        .add_user(&User::with_username("user1"), Some("pass1"))
        .await
        .expect("Could not add user");

    let admin = &mut User::with_username("admin");
    admin.roles.add("admin");
    users
        .add_user(admin, Some("admin"))
        .await
        .expect("Could not add user");
}
