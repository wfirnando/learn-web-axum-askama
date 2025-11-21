use axum::body::Body;
use axum::{routing::get, Router, http::Request, response::Response};
use tower_http::{services::ServeDir, trace::TraceLayer, classify::ServerErrorsFailureClass};
use tracing::Span;
use std::time::Duration;
use crate::handlers::
    {
     auth::{signup_handler,login_handler, post_sign_up_handler}, 
     public::home,
     todos::{create_todo_handler, todos_handler}
    };
pub fn routes() -> Router {
    let server_dir = ServeDir::new("static");
    let app = Router::new()
        .route("/",get(home))
        .route("/create", get(create_todo_handler))
        .route("/todos", get(todos_handler))
        .route("/signup", get(signup_handler)
            .post(post_sign_up_handler))
        .route("/login", get(login_handler))
        .nest_service("/static",server_dir)
        .layer(TraceLayer::new_for_http()
                   .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
                   .on_request(on_request)
                   .on_response(on_response)
                    .on_failure(on_failure),
        );

    app
}

fn on_request(request: &Request<Body>, _ : &Span){
    tracing::info!(
        "request started: method {} path {}",
        request.method(),
        request.uri().path()
    )
}
fn on_response(response: &Response<Body>, latency: Duration, _: &Span){
    tracing::info!(
        "Response generated: status {} in {:?}",
        response.status(),
        latency
    )
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _:&Span){
    tracing::error!(
        "Request Failed: {:?} after {:?}",
        error,
        latency
    )
}