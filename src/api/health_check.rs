use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde_json::json;

#[utoipa::path(
    get,
    path = "/healthcheck",
    responses(
        (status = 200, description = "Hit cl-backend succesfully")
    )
)]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy"}))).into_response()
}
