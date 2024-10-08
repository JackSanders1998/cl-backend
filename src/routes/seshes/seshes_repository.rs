pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{
    CreateSesh, Sesh, SeshSearchParams, SqlxSeshWithLocationAndClimbs, UpdateSesh,
};
use crate::routes::AppState;
use sqlx::postgres::PgQueryResult;
use sqlx::Error as PgError;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_sesh(
    state: Arc<AppState>,
    payload: CreateSesh,
    user_id: String,
) -> Result<Sesh, PgError> {
    sqlx::query_as(
        r#"
            INSERT INTO seshes (
                user_id,
                location_id,
                notes
            ) VALUES ($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(payload.location_id)
    .bind(payload.notes)
    .fetch_one(&state.db)
    .await
}

pub async fn get_sesh_by_sesh_id(state: Arc<AppState>, sesh_id: Uuid) -> Result<Sesh, PgError> {
    sqlx::query_as!(Sesh, "SELECT * FROM seshes WHERE sesh_id = $1", sesh_id)
        .fetch_one(&state.db)
        .await
}

pub async fn search_seshes(
    state: Arc<AppState>,
    params: SeshSearchParams,
) -> Result<Vec<Sesh>, PgError> {
    match params.notes {
        Some(notes) => {
            let formatted_name = "%".to_owned() + &*notes + "%";
            sqlx::query_as!(
                Sesh,
                "SELECT * FROM seshes WHERE notes LIKE $1",
                formatted_name
            )
            .fetch_all(&state.db)
            .await
        }
        None => {
            sqlx::query_as!(Sesh, "SELECT * FROM seshes")
                .fetch_all(&state.db)
                .await
        }
    }
}

pub async fn get_all_active_sesh_data(
    state: Arc<AppState>,
    user_id: String,
) -> Result<Vec<SqlxSeshWithLocationAndClimbs>, PgError> {
    sqlx::query_as(
        r#"
            WITH latest_active_sesh AS (
                SELECT * FROM seshes WHERE seshes.end IS NULL AND user_id = $1 ORDER BY created_at DESC
            )
            SELECT
                latest_active_sesh.*,
                climbs.climb_type,
                climbs.style,
                climbs.scale,
                climbs.grade,
                climbs.notes AS climb_notes,
                climbs.pointer,
                climbs.attempt,
                locations.name,
                locations.environment
            FROM latest_active_sesh
            JOIN locations ON locations.location_id = latest_active_sesh.location_id
            LEFT JOIN climbs ON climbs.sesh_id = latest_active_sesh.sesh_id;
        "#,
    )
        .bind(
            user_id.clone()
        )
        .fetch_all(&state.db)
        .await
}

pub async fn update_sesh_by_sesh_id(
    state: Arc<AppState>,
    sesh_id: Uuid,
    payload: UpdateSesh,
) -> Result<Sesh, PgError> {
    // let mut result: Result<Sesh, PgError>;
    if payload.end_session == Option::from(true) {
        sqlx::query_as!(
            Sesh,
            r#"
                UPDATE seshes
                SET location_id = COALESCE($1, location_id),
                    notes = COALESCE($2, notes),
                    "end" = CURRENT_TIMESTAMP
                WHERE sesh_id = $3
                AND seshes.end IS NULL
                RETURNING *
            "#,
            payload.location_id,
            payload.notes,
            sesh_id
        )
        .fetch_one(&state.db)
        .await
    } else {
        sqlx::query_as!(
            Sesh,
            r#"
                UPDATE seshes
                SET location_id = COALESCE($1, location_id),
                    notes = COALESCE($2, notes)
                WHERE sesh_id = $3
           RETURNING *
        "#,
            payload.location_id,
            payload.notes,
            sesh_id
        )
        .fetch_one(&state.db)
        .await
    }
}

pub async fn delete_sesh(state: Arc<AppState>, sesh_id: Uuid) -> Result<PgQueryResult, PgError> {
    sqlx::query!("DELETE FROM seshes WHERE sesh_id = $1", sesh_id)
        .execute(&state.db)
        .await
}
