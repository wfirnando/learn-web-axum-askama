use axum::response::{Html, Response, IntoResponse};
use crate::models::templates::{SignUpTemplate, LogInTemplate};
use askama::Template;


pub async fn signup_handler() -> Response {
    let html_string =   SignUpTemplate{}.render().unwrap();
    Html(html_string).into_response()
}

pub async fn login_handler() -> Response {
    let html_string =   LogInTemplate{}.render().unwrap();
    Html(html_string).into_response()
}
