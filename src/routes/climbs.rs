pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{Climb, CreateClimb};
use crate::routes::AppState;
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_climb(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateClimb>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
            INSERT INTO climbs (
                sesh_id,
                climb_type,
                style,
                scale,
                grade,
                attempt,
                pointer,
                notes
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                climb_id,
                sesh_id,
                climb_type as "climb_type!: ClimbType",
                style as "style!: Style",
                scale as "scale!: Scale",
                grade,
                attempt as "attempt!: Attempt",
                pointer,
                notes,
                created_at,
                updated_at
        "#,
        payload.sesh_id,
        payload.climb_type as _,
        payload.style as _,
        payload.scale as _,
        payload.grade,
        payload.attempt as _,
        payload.pointer,
        payload.notes,
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(climb) => (
            StatusCode::CREATED,
            Json(json!({
                "climbId": climb.climb_id,
                "seshId": climb.sesh_id,
                "climbType": climb.climb_type,
                "style": climb.style,
                "scale": climb.scale,
                "grade": climb.grade,
                "attempt": climb.attempt,
                "pointer": climb.pointer,
                "notes": climb.notes,
                "createdAt": climb.created_at,
                "updatedAt": climb.updated_at,
            })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_climb_by_climb_id(
    State(state): State<Arc<AppState>>,
    Path(climb_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Climb,
        r#"
            SELECT
                climb_id,
                sesh_id,
                climb_type as "climb_type: ClimbType",
                style as "style: Style",
                scale as "scale: Scale",
                grade,
                attempt as "attempt: Attempt",
                pointer,
                notes,
                created_at,
                updated_at
            FROM climbs
            WHERE climb_id = $1
        "#,
        climb_id
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(climb) => (StatusCode::OK, Json(climb)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn search_climbs(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Climb,
        r#"
            SELECT
                climb_id,
                sesh_id,
                climb_type as "climb_type: ClimbType",
                style as "style: Style",
                scale as "scale: Scale",
                grade,
                attempt as "attempt: Attempt",
                pointer,
                notes,
                created_at,
                updated_at
            FROM climbs
        "#
    )
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(climb) => (StatusCode::OK, Json(climb)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_climb(
    State(state): State<Arc<AppState>>,
    Path(climb_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM climbs WHERE climb_id = $1", climb_id)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
