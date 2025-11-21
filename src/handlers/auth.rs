
use axum::
    {response::{Html, Response, IntoResponse, Redirect},
    Form,
    };

use crate::models::{
    templates::{SignUpTemplate, LogInTemplate, },
    user_form_model::UserFormModel};

use askama::Template;

pub async fn signup_handler() -> Response {
    let html_string =   SignUpTemplate{}.render().unwrap();
    Html(html_string).into_response()
}

pub async  fn post_sign_up_handler(Form(user_form): Form<UserFormModel>)-> Response {
    tracing::info!("Email is {} and password {}", user_form.email, user_form.password);
    Redirect::to("/").into_response()
}
pub async fn login_handler() -> Response {
    let html_string =   LogInTemplate{}.render().unwrap();
    Html(html_string).into_response()
}
