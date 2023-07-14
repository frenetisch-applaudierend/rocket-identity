use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_identity::auth::{UserData, UserRepository};

pub fn routes() -> Vec<rocket::Route> {
    routes![register]
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

    #[response(status = 400)]
    Failure(()),
}
