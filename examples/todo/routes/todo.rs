use rocket::{State, serde::{json::Json, Serialize}};
use rocket_identity::auth::User;
use tokio::sync::RwLock;

use crate::services::TodoService;

pub fn routes() -> Vec<rocket::Route> {
    routes![list, details, create]
}

#[get("/")]
async fn list<'r>(
    user: &User,
    todos: &'r State<RwLock<TodoService>>,
) -> TodoResponse<Vec<TodoListHeader>> {
    let todos = todos.read().await;

    let lists = todos
        .get_all_lists()
        .iter()
        .map(|list| TodoListHeader {
            id: list.id.clone(),
            name: list.name.clone(),
            items: list.items.len(),
        })
        .collect::<Vec<_>>();

    TodoResponse::Success(Json(lists))
}

#[get("/<id>")]
fn details(user: &User, id: &str) -> String {
    format!("Hello, {}! Here is list {}", user.username(), id)
}

#[post("/")]
async fn create(user: &User, todos: &State<RwLock<TodoService>>) -> String {
    let mut todos = todos.write().await;

    let created_list = todos.create_list("Test".to_string());
    format!("Hello, {}! I created a new list for you", user.username())
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoListHeader {
    pub id: uuid::Uuid,
    pub name: String,
    pub items: usize,
}

#[derive(Responder)]
pub enum TodoResponse<T> {
    #[response(status = 200)]
    Success(Json<T>),
    #[response(status = 404)]
    NotFound(()),
    #[response(status = 500)]
    Failure(()),
}
