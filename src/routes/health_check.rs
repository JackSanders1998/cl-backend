use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy"}))).into_response()
}
