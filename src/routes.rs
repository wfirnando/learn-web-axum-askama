use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use crate::handlers::{auth::{signup_handler,login_handler}, public::home, todos::{create_todo_handler, todos_handler}};
pub fn routes() -> Router {
    let server_dir = ServeDir::new("static");
    let app = Router::new()
        .route("/",get(home))
        .route("/create", get(create_todo_handler))
        .route("/todos", get(todos_handler))
        .route("/signup", get(signup_handler))
        .route("/login", get(login_handler))
        .nest_service("/static",server_dir);

    app
}