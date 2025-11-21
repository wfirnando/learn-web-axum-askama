use web_axum_askama::{routes, init};


#[tokio::main]
async fn main() {

    let addr   = "127.0.0.1:8000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind addr");


    //logging
    init::logging();
    tracing::info!("server is starting...");
    tracing::info!("listening on http://{}", addr);

    let app= routes::routes();
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");


}




