pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{CreateSesh, SeshSearchParams, SqlxSeshWithLocationAndClimbs, UpdateSesh};
use crate::routes::AppState;
use sqlx::postgres::PgQueryResult;
use sqlx::Error as PgError;
use sqlx::FromRow;
use std::sync::Arc;
use tracing::info;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Copy, FromRow, ToSchema)]
pub struct Id {
    pub(crate) sesh_id: Uuid,
}

pub async fn create_sesh(
    state: Arc<AppState>,
    payload: CreateSesh,
    user_id: String,
) -> Result<Id, PgError> {
    info!(
        "create_sesh called by {} with payload: {:?}",
        user_id,
        payload
    );

    sqlx::query_as(
        r#"
            INSERT INTO seshes (
                user_id,
                location_id,
                notes
            ) VALUES ($1, $2, $3)
            RETURNING sesh_id
        "#,
    )
    .bind(user_id)
    .bind(payload.location_id)
    .bind(payload.notes)
    .fetch_one(&state.db)
    .await
}

pub async fn reconcile_active_seshes(
    state: Arc<AppState>,
    user_id: String,
) -> Result<Id, PgError> {
    info!(
        "reconcile_active_seshes called by {}",
        user_id,
    );

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

pub async fn search_seshes(
    state: Arc<AppState>,
    params: SeshSearchParams,
    user_id: String,
) -> Result<Vec<Id>, PgError> {
    match params.notes {
        Some(notes) => {
            let formatted_name = "%".to_owned() + &*notes + "%";
            sqlx::query_as!(
                Id,
                "SELECT sesh_id FROM seshes WHERE user_id = $1 AND notes LIKE $2 ORDER BY start DESC;",
                user_id,
                formatted_name
            )
            .fetch_all(&state.db)
            .await
        }
        None => {
            sqlx::query_as!(Id, "SELECT sesh_id FROM seshes WHERE user_id = $1 ORDER BY start DESC;", user_id)
                .fetch_all(&state.db)
                .await
        }
    }
}

pub async fn get_seshes_with_location_and_climbs(
    state: Arc<AppState>,
    sesh_ids: Vec<Uuid>,
) -> Result<Vec<SqlxSeshWithLocationAndClimbs>, PgError> {
    let sesh_ids_string: Vec<String> = sesh_ids
        .iter()
        .map(|id| format!("'{}'", id.to_string()))
        .collect();
    let query_string = format!(
        r#"
            WITH sesh_data AS (
                SELECT * FROM seshes WHERE seshes.sesh_id IN ( {} ) ORDER BY start DESC
            )
            SELECT
                sesh_data.*,
                climbs.climb_id,
                climbs.climb_type,
                climbs.style,
                climbs.scale,
                climbs.grade,
                climbs.attempt,
                climbs.pointer,
                climbs.notes AS climb_notes,
                climbs.created_at AS climb_created_at,
                climbs.updated_at AS climb_updated_at,
                locations.location_id,
                locations.name,
                locations.environment,
                locations.created_at AS location_created_at,
                locations.updated_at AS location_updated_at
            FROM sesh_data
            JOIN locations ON locations.location_id = sesh_data.location_id
            LEFT JOIN climbs ON climbs.sesh_id = sesh_data.sesh_id
            ORDER BY sesh_data.created_at DESC;
        "#,
        sesh_ids_string.join(",")
    );

    let mut query = sqlx::query_as(&query_string);
    for sesh_id in sesh_ids {
        query = query.bind(sesh_id);
    }

    query.fetch_all(&state.db).await
}

pub async fn get_all_active_sesh_data(
    state: Arc<AppState>,
    user_id: String,
) -> Result<Vec<SqlxSeshWithLocationAndClimbs>, PgError> {
    sqlx::query_as(
        r#"
            WITH latest_active_sesh AS (
                SELECT * FROM seshes WHERE seshes.end IS NULL AND user_id = $1 ORDER BY start DESC LIMIT 1
            )
            SELECT
                latest_active_sesh.*,
                climbs.climb_id,
                climbs.climb_type,
                climbs.style,
                climbs.scale,
                climbs.grade,
                climbs.attempt,
                climbs.pointer,
                climbs.notes AS climb_notes,
                climbs.created_at AS climb_created_at,
                climbs.updated_at AS climb_updated_at,
                locations.location_id,
                locations.name,
                locations.environment,
                locations.created_at AS location_created_at,
                locations.updated_at AS location_updated_at
            FROM latest_active_sesh
            JOIN locations ON locations.location_id = latest_active_sesh.location_id
            LEFT JOIN climbs ON climbs.sesh_id = latest_active_sesh.sesh_ids
            ORDER BY latest_active_sesh.start DESC;
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
