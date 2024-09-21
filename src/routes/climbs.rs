use crate::models::{Climb, CreateClimb};
pub use crate::models::{ClimbType, Scale, Style};
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
        r#"INSERT INTO climbs (sesh_id, climb_type, style, scale, grade) VALUES ($1, $2, $3, $4, $5) RETURNING climb_id, sesh_id, climb_type as "climb_type!: ClimbType", style as "style!: Style", scale as "scale!: Scale", grade, created_at, updated_at"#,
        payload.sesh_id,
        payload.climb_type as _,
        payload.style as _,
        payload.scale as _,
        payload.grade,
    )
        .fetch_one(&pool)
        .await;

    match result {
        Ok(climb) => (
            StatusCode::CREATED,
            Json(json!({
                "climb_id": climb.climb_id,
                "sesh_id": climb.sesh_id,
                "climb_type": climb.climb_type,
                "style": climb.style,
                "scale": climb.scale,
                "grade": climb.grade,
                "created_at": climb.created_at,
                "updated_at": climb.updated_at,
            })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_climb(
    State(pool): State<PgPool>,
    Path(climb_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(Climb, r#"SELECT climb_id, sesh_id, climb_type as "climb_type: ClimbType", style as "style: Style", scale as "scale: Scale", grade, created_at, updated_at FROM climbs WHERE climb_id = $1"#, climb_id)
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
