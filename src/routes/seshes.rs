use crate::models::{CreateSesh, Sesh};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_sesh(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateSesh>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO seshes (user_id, location_id, notes) VALUES ($1, $2, $3) RETURNING sesh_id",
        payload.user_id,
        payload.location_id,
        payload.notes
    )
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => (
            StatusCode::CREATED,
            Json(json!({ "sesh_id": record.sesh_id })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_sesh(
    State(pool): State<PgPool>,
    Path(sesh_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Sesh,
        "SELECT * FROM seshes WHERE sesh_id = $1",
        sesh_id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(sesh) => (StatusCode::OK, Json(sesh)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_sesh(
    State(pool): State<PgPool>,
    Path(sesh_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM seshes WHERE sesh_id = $1", sesh_id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
