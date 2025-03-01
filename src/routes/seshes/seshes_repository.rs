use crate::models::{CreateSesh, SeshFromRow, SeshRow, SeshSearchParams, UpdateSesh};
use crate::routes::AppState;
use sqlx::postgres::PgQueryResult;
use sqlx::Error as PgError;
use sqlx::FromRow;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

#[derive(Clone, Copy, FromRow)]
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

pub async fn search_seshes(
    state: Arc<AppState>,
    params: SeshSearchParams,
    user_id: String,
) -> Result<Vec<Id>, PgError> {
    match params.notes {
        Some(notes) => {
            let formatted_name = "%".to_owned() + &*notes + "%";
            sqlx::query_as::<_, Id>(
                "SELECT sesh_id FROM seshes WHERE user_id = $1 AND notes LIKE $2 ORDER BY start DESC;"
            )
            .bind(user_id)
            .bind(formatted_name)
            .fetch_all(&state.db)
            .await
        }
        None => {
            sqlx::query_as!(
                Id,
                "SELECT sesh_id FROM seshes WHERE user_id = $1 ORDER BY start DESC;",
                user_id
            )
            .fetch_all(&state.db)
            .await
        }
    }
}

pub async fn get_sesh_and_location_by_id(
    state: Arc<AppState>,
    sesh_id: Uuid,
) -> Result<SeshFromRow, PgError> {
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
            WHERE sesh_id = $1
            ORDER BY seshes.created_at DESC;
        "#,
    )
    .bind(sesh_id)
    .fetch_one(&state.db)
    .await
}

pub async fn get_hydrated_seshes(
    state: Arc<AppState>,
    sesh_ids: Vec<Uuid>,
) -> Result<Vec<SeshFromRow>, PgError> {
    let sesh_ids_string: Vec<String> = sesh_ids
        .iter()
        .map(|id| format!("'{}'", id.to_string()))
        .collect();
    let query_string = format!(
        r#"
            WITH sesh_data AS (
                SELECT * FROM seshes WHERE seshes.sesh_id IN ( {} ) ORDER BY start DESC
            ), tick_and_location_data AS (
              SELECT
                sesh_data.*,

                locations.location_id,
                locations.author,
                locations.name,
                locations.environment,
                locations.description,
                locations.created_at AS location_created_at,
                locations.updated_at AS location_updated_at,

                ticks.tick_id,
                ticks.route_id,
                ticks.discipline,
                ticks.attempt,
                ticks.notes AS tick_notes,
                ticks.lap_group,
                ticks.created_at AS tick_created_at,
                ticks.updated_at AS tick_updated_at
            FROM sesh_data
              JOIN locations ON locations.location_id = sesh_data.location_id
              LEFT JOIN ticks ON ticks.sesh_id = sesh_data.sesh_id
            ), route_data AS (
              SELECT 
              tick_and_location_data.*,

              routes.route_id,
              routes.location_id,
              routes.grade,
              routes.scale,
              routes.disciplines,
              routes.author AS route_author,
              routes.description AS route_description,
              routes.created_at AS route_created_at,
              routes.updated_at AS route_updated_at
            FROM tick_and_location_data
                JOIN routes ON routes.route_id = tick_and_location_data.route_id
            )
            SELECT * FROM route_data
            ORDER BY route_data.created_at DESC;
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
) -> Result<Vec<SeshFromRow>, PgError> {
    sqlx::query_as(
        r#"
            WITH latest_active_sesh AS (
                SELECT * FROM seshes WHERE seshes.end IS NULL AND user_id = $1 ORDER BY start DESC LIMIT 1
            )
            SELECT
                latest_active_sesh.*,
                routes.climb_id,
                routes.climb_type,
                routes.style,
                routes.scale,
                routes.grade,
                routes.attempt,
                routes.pointer,
                routes.notes AS climb_notes,
                routes.created_at AS climb_created_at,
                routes.updated_at AS climb_updated_at,
                locations.location_id,
                locations.name,
                locations.environment,
                locations.created_at AS location_created_at,
                locations.updated_at AS location_updated_at
            FROM latest_active_sesh
            JOIN locations ON locations.location_id = latest_active_sesh.location_id
            LEFT JOIN climbs ON routes.sesh_id = latest_active_sesh.sesh_id
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
