use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

pub async fn get() -> impl IntoResponse {
    Json(json!({ "url": "ws://127.0.0.1:1140/" }))
}
