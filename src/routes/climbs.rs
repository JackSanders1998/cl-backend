use crate::models::{CreateClimb, Climb};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_climb(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateClimb>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO climbs (sesh_id, climb_type, style, scale, grade) VALUES ($1, $2, $3, $4, $5) RETURNING climb_id",
        payload.sesh_id,
        payload.climb_type,
        payload.style,
        payload.scale,
        payload.grade,
    )
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => (
            StatusCode::CREATED,
            Json(json!({ "climb_id": record.climb_id })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_climb(
    State(pool): State<PgPool>,
    Path(climb_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Climb,
        "SELECT * FROM climbs WHERE climb_id = $1",
        climb_id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(climb) => (StatusCode::OK, Json(climb)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_climb(
    State(pool): State<PgPool>,
    Path(climb_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM climbs WHERE climb_id = $1", climb_id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
