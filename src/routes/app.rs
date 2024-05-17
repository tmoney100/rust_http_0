use crate::handlers::hello_world::handle_hello;
use axum::{routing::get, Router};

pub fn build_routes() -> Router {
    let app = Router::new().route("/hello", get(handle_hello));

    app
}
