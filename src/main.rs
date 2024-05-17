use axum;
use tokio;

/// Route handlers.
mod handlers;
/// Public HTTP API.
mod routes;

#[tokio::main]
async fn main() {
    let app_routes = routes::app::build_routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app_routes).await.unwrap();
}
