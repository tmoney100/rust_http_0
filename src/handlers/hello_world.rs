use crate::routes::utils::json::JsonResponse;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn handle_hello() -> impl IntoResponse {
    let response = JsonResponse {
        message: "Hello, world!".to_string(),
    };
    (StatusCode::OK, Json(response))
}
