use crate::models::{CreateSesh, SeshRow, SeshWithLocationSqlx, UpdateSesh};
use crate::routes::AppState;
use sqlx::postgres::PgQueryResult;
use sqlx::Error as PgError;
use sqlx::FromRow;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Clone, Copy, FromRow, Debug)]
pub struct Id {
    pub(crate) sesh_id: Uuid,
}

pub async fn create_sesh(
    state: Arc<AppState>,
    payload: CreateSesh,
    user_id: String,
) -> Result<SeshRow, PgError> {
    info!(
        "create_sesh called by {} with payload: {:?}",
        user_id, payload
    );

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

pub async fn get_sesh_and_location_by_sesh_id(
    state: Arc<AppState>,
    sesh_id: Uuid,
) -> Result<SeshWithLocationSqlx, PgError> {
    sqlx::query_as(
        r#"
              SELECT
                seshes.*,
                locations.location_id,
                locations.author,
                locations.name,
                locations.environment,
                locations.description,
                locations.created_at AS location_created_at,
                locations.updated_at AS location_updated_at
            FROM seshes
              JOIN locations ON locations.location_id = seshes.location_id
            WHERE sesh_id = $1;
        "#,
    )
    .bind(sesh_id)
    .fetch_one(&state.db)
    .await
}

pub async fn search_seshes(
    state: Arc<AppState>,
    user_id: String,
) -> Result<Vec<SeshWithLocationSqlx>, PgError> {
    sqlx::query_as(
        r#"
              SELECT
                seshes.*,
                locations.author,
                locations.name,
                locations.environment,
                locations.description,
                locations.created_at AS location_created_at,
                locations.updated_at AS location_updated_at
            FROM seshes
              JOIN locations ON locations.location_id = seshes.location_id
            WHERE user_id = $1
            ORDER BY seshes.created_at DESC;
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await
}

pub async fn get_active_seshes(
    state: Arc<AppState>,
    user_id: String,
) -> Result<Vec<SeshWithLocationSqlx>, PgError> {
    sqlx::query_as(
        r#"
              SELECT
                seshes.*,
                locations.author,
                locations.name,
                locations.environment,
                locations.description,
                locations.created_at AS location_created_at,
                locations.updated_at AS location_updated_at
            FROM seshes
              JOIN locations ON locations.location_id = seshes.location_id
            WHERE user_id = $1
            AND seshes.end IS NULL
            ORDER BY seshes.created_at DESC;
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await
}

pub async fn reconcile_active_seshes(state: Arc<AppState>, user_id: String) -> Result<Id, PgError> {
    info!("reconcile_active_seshes called by {}", user_id,);

    sqlx::query_as(
        r#"
        WITH latest_sesh AS (
            SELECT sesh_id FROM seshes WHERE user_id = $1 ORDER BY start DESC LIMIT 1
        )
        UPDATE seshes
        SET "end" = CURRENT_TIMESTAMP
        WHERE user_id = $1 AND sesh_id NOT IN (SELECT sesh_id FROM latest_sesh) RETURNING sesh_id "#,
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await
}

pub async fn update_sesh_by_sesh_id(
    state: Arc<AppState>,
    sesh_id: Uuid,
    payload: UpdateSesh,
) -> Result<Id, PgError> {
    if payload.end_session == Option::from(true) {
        sqlx::query_as!(
            Id,
            r#"
                UPDATE seshes
                SET location_id = COALESCE($1, location_id),
                    notes = COALESCE($2, notes),
                    "end" = CURRENT_TIMESTAMP
                WHERE sesh_id = $3
                AND seshes.end IS NULL
                RETURNING sesh_id
            "#,
            payload.location_id,
            payload.notes,
            sesh_id
        )
        .fetch_one(&state.db)
        .await
    } else {
        sqlx::query_as!(
            Id,
            r#"
                UPDATE seshes
                SET location_id = COALESCE($1, location_id),
                    notes = COALESCE($2, notes)
                WHERE sesh_id = $3
           RETURNING sesh_id
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
