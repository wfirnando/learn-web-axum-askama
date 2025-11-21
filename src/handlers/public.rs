use axum::response::{Html, Response, IntoResponse};
use crate::models::templates::{HomeTemplate};
use askama::Template;
pub async fn home() -> Response {
    let html_string =   HomeTemplate{}.render().unwrap();
    tracing::info!("info log");
    Html(html_string).into_response()
}