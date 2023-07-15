use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_identity::auth::{
    scheme::jwt::{JwtToken, JwtTokenProvider},
    LoginError, UserData, UserRepository,
};

pub fn routes() -> Vec<rocket::Route> {
    routes![login, register]
}

#[post("/login", data = "<body>")]
async fn login<'r>(
    users: &UserRepository,
    jwt: JwtTokenProvider<'_>,
    body: Json<LoginReq<'r>>,
) -> LoginRes {
    let result = users.authenticate(body.username, body.password).await;

    match result {
        Ok(user) => {
            let token = match jwt.create_token(&user) {
                Ok(token) => token,
                Err(_) => return LoginRes::Failure(()),
            };
            LoginRes::Success(Json(LoginResData { token }))
        }
        Err(LoginError::UserNotFound)
        | Err(LoginError::IncorrectPassword)
        | Err(LoginError::MissingPassword) => LoginRes::InvalidCredentials(()),
        Err(_) => LoginRes::Failure(()),
    }
}

#[post("/register", data = "<body>")]
async fn register<'r>(users: &UserRepository, body: Json<RegisterReq<'r>>) -> RegisterRes<'r> {
    let user_data = UserData::with_username(body.username);
    let result = users.add_user(user_data, Some(body.password)).await;

    match result {
        Ok(_) => RegisterRes::Success(Json(RegisterResData {
            username: body.username,
        })),
        Err(_) => RegisterRes::Failure(()),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct LoginReq<'r> {
    username: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct LoginResData {
    token: JwtToken,
}

#[derive(Responder)]
enum LoginRes {
    #[response(status = 200)]
    Success(Json<LoginResData>),

    #[response(status = 401)]
    InvalidCredentials(()),

    #[response(status = 500)]
    Failure(()),
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RegisterReq<'r> {
    username: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RegisterResData<'r> {
    username: &'r str,
}

#[derive(Responder)]
enum RegisterRes<'r> {
    #[response(status = 200)]
    Success(Json<RegisterResData<'r>>),

    #[response(status = 500)]
    Failure(()),
}
