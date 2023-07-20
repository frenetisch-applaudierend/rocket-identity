#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

mod task;
#[cfg(test)]
mod tests;
mod user;

use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::serde::Serialize;
use rocket::{Build, Rocket};

use rocket_dyn_templates::{context, Template};

use rocket_identity::schemes::cookie::{CookieScheme, CookieSession};
use rocket_identity::stores::InMemoryUserStore;
use rocket_identity::{Identity, User, UserRepository};

use crate::task::{Task, Todo};
use crate::user::{Login, Registration};

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Context {
    flash: Option<(String, String)>,
    tasks: Vec<Task>,
}

impl Context {
    pub async fn err<M: std::fmt::Display>(conn: &DbConn, user: &User, msg: M) -> Context {
        Context {
            flash: Some(("error".into(), msg.to_string())),
            tasks: Task::all(conn, user).await.unwrap_or_default(),
        }
    }

    pub async fn raw(conn: &DbConn, user: &User, flash: Option<(String, String)>) -> Context {
        match Task::all(conn, user).await {
            Ok(tasks) => Context { flash, tasks },
            Err(e) => {
                error_!("DB Task::all() error: {}", e);
                Context {
                    flash: Some(("error".into(), "Fail to access database.".into())),
                    tasks: vec![],
                }
            }
        }
    }
}

#[post("/", data = "<todo_form>")]
async fn new(todo_form: Form<Todo>, conn: DbConn, user: &User) -> Flash<Redirect> {
    let todo = todo_form.into_inner();
    if todo.description.is_empty() {
        Flash::error(Redirect::to("/"), "Description cannot be empty.")
    } else if let Err(e) = Task::insert(todo, &conn, user).await {
        error_!("DB insertion error: {}", e);
        Flash::error(
            Redirect::to("/"),
            "Todo could not be inserted due an internal error.",
        )
    } else {
        Flash::success(Redirect::to("/"), "Todo successfully added.")
    }
}

#[put("/<id>")]
async fn toggle(id: i32, conn: DbConn, user: &User) -> Result<Redirect, Template> {
    match Task::toggle_with_id(id, &conn).await {
        Ok(_) => Ok(Redirect::to("/")),
        Err(e) => {
            error_!("DB toggle({}) error: {}", id, e);
            Err(Template::render(
                "index",
                Context::err(&conn, user, "Failed to toggle task.").await,
            ))
        }
    }
}

#[delete("/<id>")]
async fn delete(id: i32, conn: DbConn, user: &User) -> Result<Flash<Redirect>, Template> {
    match Task::delete_with_id(id, &conn).await {
        Ok(_) => Ok(Flash::success(Redirect::to("/"), "Todo was deleted.")),
        Err(e) => {
            error_!("DB deletion({}) error: {}", id, e);
            Err(Template::render(
                "index",
                Context::err(&conn, user, "Failed to delete task.").await,
            ))
        }
    }
}

#[get("/")]
async fn index(flash: Option<FlashMessage<'_>>, conn: DbConn, user: &User) -> Template {
    let flash = flash.map(FlashMessage::into_inner);
    Template::render("index", Context::raw(&conn, user, flash).await)
}

#[get("/login")]
fn login_page() -> Template {
    Template::render("login", context! {})
}

#[post("/login", data = "<login_form>")]
async fn login(
    login_form: Form<Login<'_>>,
    users: &UserRepository,
    session: CookieSession<'_>,
) -> Result<Redirect, Template> {
    let user = users
        .authenticate(login_form.username, login_form.password)
        .await
        .map_err(|e| {
            error_!("UserRepository::authenticate() error: {}", e);
            Template::render("login", context! {})
        })?;

    session.sign_in(&user);

    Ok(Redirect::to("/"))
}

#[get("/register")]
fn register_page() -> Template {
    Template::render("register", context! {})
}

#[post("/register", data = "<registration_form>")]
async fn register(
    registration_form: Form<Registration<'_>>,
    users: &UserRepository,
    session: CookieSession<'_>,
) -> Result<Redirect, Template> {
    let user = User::with_username(registration_form.username);
    users
        .add_user(&user, Some(registration_form.password))
        .await
        .map_err(|e| {
            error_!("UserRepository::add_user() error: {}", e);
            Template::render("register", context! {})
        })?;

    session.sign_in(&user);

    Ok(Redirect::to("/"))
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    DbConn::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("diesel migrations");
        })
        .await;

    rocket
}

#[launch]
fn rocket() -> _ {
    let identity_config = Identity::config()
        .with_user_store(InMemoryUserStore::new())
        .add_scheme(CookieScheme::default())
        .build();

    rocket::build()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .attach(Identity::fairing(identity_config))
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index])
        .mount("/todo", routes![new, toggle, delete])
        .mount("/user", routes![login_page, login, register_page, register])
}
