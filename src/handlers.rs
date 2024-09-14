use crate::models::{CreateLocation, Location};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse, Extension};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_location(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLocation>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO locations (user_id, name, environment) VALUES ($1, $2, $3) RETURNING location_id",
        payload.user_id,
        payload.name,
        payload.environment
    )
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => (
            StatusCode::CREATED,
            Json(json!({ "location_id": record.location_id })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_location(
    State(pool): State<PgPool>,
    Path(location_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Location,
        "SELECT * FROM locations WHERE location_id = $1",
        location_id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(location) => (StatusCode::OK, Json(location)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_location(
    State(pool): State<PgPool>,
    Path(location_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM locations WHERE location_id = $1", location_id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
