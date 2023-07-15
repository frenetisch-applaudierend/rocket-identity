use jsonwebtoken::{DecodingKey, EncodingKey};
use rocket_identity::{
    auth::scheme::jwt::{JwtBearer, JwtConfig},
    config::Config,
    persistence::store::InMemoryUserStore,
    RocketIdentity,
};

#[macro_use]
extern crate rocket;

mod routes;

#[launch]
fn rocket() -> _ {
    let user_store = InMemoryUserStore::new();

    // This should be read from configuration
    let secret = b"My Secret";
    let jwt_config = JwtConfig {
        encoding_key: EncodingKey::from_secret(secret),
        deconding_key: DecodingKey::from_secret(secret),
    };

    let identity_config = Config::new(user_store).add_scheme(JwtBearer::new(jwt_config));

    rocket::build()
        .attach(RocketIdentity::fairing(identity_config))
        .mount("/users", routes::users::routes())
        .mount("/todo", routes::todo::routes())
}
