use crate::models::{CreateLocation, Location};
use crate::routes::{get_claims, AppState};
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_location(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateLocation>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO locations (user_id, name, environment) VALUES ($1, $2, $3) RETURNING *",
        get_claims(headers),
        payload.name,
        payload.environment
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(location) => (
            StatusCode::CREATED,
            Json(json!({
               "location_id": location.location_id,
               "user_id":  location.user_id,
                "name": location.name,
                "environment": location.environment,
                "created_at":  location.created_at,
                "updated_at":  location.updated_at,
            })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_location(
    State(state): State<Arc<AppState>>,
    Path(location_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Location,
        "SELECT * FROM locations WHERE location_id = $1",
        location_id
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(location) => (StatusCode::OK, Json(location)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_location(
    State(state): State<Arc<AppState>>,
    Path(location_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM locations WHERE location_id = $1", location_id)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
