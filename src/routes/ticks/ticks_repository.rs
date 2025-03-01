use std::sync::Arc;

use crate::{
    models::{CreateTick, TickRow},
    routes::AppState,
};
use sqlx::Error as PgError;
use uuid::Uuid;

pub async fn create_tick(state: Arc<AppState>, payload: CreateTick) -> Result<TickRow, PgError> {
    sqlx::query_as(
        r#"
            INSERT INTO ticks (
                sesh_id,
                route_id,
                discipline,
                attempt,
                notes,
                lap_group
            ) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
        "#,
    )
    .bind(payload.sesh_id)
    .bind(payload.route_id)
    .bind(payload.discipline)
    .bind(payload.attempt)
    .bind(payload.notes)
    .bind(payload.lap_group)
    .fetch_one(&state.db)
    .await
}

pub async fn search_ticks(state: Arc<AppState>, sesh_id: Uuid) -> Result<Vec<TickRow>, PgError> {
    sqlx::query_as("SELECT * FROM ticks WHERE sesh_id = $1")
        .bind(sesh_id)
        .fetch_all(&state.db)
        .await
}
