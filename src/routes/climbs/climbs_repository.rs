pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{Climb, CreateClimb};
use crate::routes::AppState;
use sqlx::postgres::PgQueryResult;
use sqlx::Error as PgError;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_climb(state: Arc<AppState>, payload: CreateClimb) -> Result<Climb, PgError> {
    sqlx::query_as(
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
    )
    .bind(payload.sesh_id)
    .bind(payload.climb_type)
    .bind(payload.style)
    .bind(payload.scale)
    .bind(payload.grade)
    .bind(payload.attempt)
    .bind(payload.pointer)
    .bind(payload.notes)
    .fetch_one(&state.db)
    .await
}

pub async fn get_climb_by_climb_id(state: Arc<AppState>, climb_id: Uuid) -> Result<Climb, PgError> {
    sqlx::query_as!(
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
    .await
}

pub async fn get_all_climbs(state: Arc<AppState>) -> Result<Vec<Climb>, PgError> {
    sqlx::query_as!(
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
    .await
}

pub async fn delete_climb(state: Arc<AppState>, climb_id: Uuid) -> Result<PgQueryResult, PgError> {
    sqlx::query!("DELETE FROM climbs WHERE climb_id = $1", climb_id)
        .execute(&state.db)
        .await
}
