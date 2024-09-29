pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{
    ClimbData, CreateLocation, CreateSesh, Sesh, SeshWithLocationAndClimbs,
    SqlxSeshWithLocationAndClimbs, UpdateSesh,
};
use crate::routes::{get_claims, AppState};
use axum::extract::{Path, Query, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use http::HeaderMap;
use serde_json::json;
use sqlx::Error;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_sesh(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSesh>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO seshes (user_id, location_id, notes) VALUES ($1, $2, $3) RETURNING *",
        get_claims(headers),
        payload.location_id,
        payload.notes
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(sesh) => (
            StatusCode::CREATED,
            Json(json!({
                "seshId": sesh.sesh_id,
                "userId": sesh.user_id,
                "locationId": sesh.location_id,
                "notes": sesh.notes,
                "start": sesh.start,
                "end": sesh.end,
                "createdAt": sesh.created_at,
                "updatedAt": sesh.updated_at,
            })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_sesh(
    State(state): State<Arc<AppState>>,
    Path(sesh_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(Sesh, "SELECT * FROM seshes WHERE sesh_id = $1", sesh_id)
        .fetch_one(&state.db)
        .await;

    match result {
        Ok(sesh) => (StatusCode::OK, Json(sesh)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn search_seshes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let result;
    let notes = params.get("notes");
    if notes.is_some() {
        let formatted_name = "%".to_owned() + notes.unwrap() + "%";
        result = sqlx::query_as!(
            Sesh,
            "SELECT * FROM seshes WHERE notes LIKE $1",
            formatted_name
        )
        .fetch_all(&state.db)
        .await;
    } else {
        result = sqlx::query_as!(Sesh, "SELECT * FROM seshes")
            .fetch_all(&state.db)
            .await;
    }

    match result {
        Ok(sesh) => (StatusCode::OK, Json(sesh)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

fn map_db_rows_to_sesh_object(
    sesh: Vec<SqlxSeshWithLocationAndClimbs>,
) -> SeshWithLocationAndClimbs {
    let location = CreateLocation {
        name: sesh[0].name.clone(),
        environment: sesh[0].environment.clone(),
    };

    let mut sesh_with_location_and_climbs = SeshWithLocationAndClimbs {
        sesh_id: sesh[0].sesh_id,
        user_id: sesh[0].user_id.clone(),
        location_id: sesh[0].location_id,
        notes: sesh[0].notes.clone(),
        start: sesh[0].start,
        end: sesh[0].end,
        created_at: sesh[0].created_at,
        updated_at: sesh[0].updated_at,
        location,
        climbs: Vec::new(),
    };

    for row in sesh {
        let climb_row = ClimbData {
            climb_type: row.climb_type,
            style: row.style,
            scale: row.scale,
            grade: row.grade,
            attempt: row.attempt,
            pointer: row.pointer,
            notes: row.notes,
        };
        sesh_with_location_and_climbs.climbs.push(climb_row);
    }

    return sesh_with_location_and_climbs;
}

pub async fn get_active_sesh(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let result = get_active_sesh_helper(headers, state).await;

    match result {
        Ok(sesh) => (StatusCode::OK, Json(map_db_rows_to_sesh_object(sesh))).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn get_active_sesh_helper(
    headers: HeaderMap,
    state: Arc<AppState>,
) -> Result<Vec<SqlxSeshWithLocationAndClimbs>, Error> {
    let result = sqlx::query_as!(
        SqlxSeshWithLocationAndClimbs,
        r#"
            WITH latest_active_sesh AS (
                SELECT * FROM seshes WHERE seshes.end IS NULL AND user_id = $1 ORDER BY created_at DESC
            )
            SELECT
                latest_active_sesh.*,
                climbs.climb_type as "climb_type: ClimbType",
                climbs.style as "style: Style",
                climbs.scale as "scale: Scale",
                climbs.grade,
                climbs.notes as climb_notes,
                climbs.pointer,
                climbs.attempt as "attempt: Attempt",
                locations.name,
                locations.environment
            FROM latest_active_sesh
            JOIN locations ON locations.location_id = latest_active_sesh.location_id
            JOIN climbs ON climbs.sesh_id = latest_active_sesh.sesh_id;
        "#,
        get_claims(headers)
    )
        .fetch_all(&state.db)
        .await;

    return result;
}

pub async fn update_sesh_by_sesh_id(
    State(state): State<Arc<AppState>>,
    Path(sesh_id): Path<Uuid>,
    Json(payload): Json<UpdateSesh>,
) -> impl IntoResponse {
    if payload.end_session == Option::from(true) {
        let result = sqlx::query!(
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
        .await;

        match result {
            Ok(sesh) => (
                StatusCode::OK,
                Json(json!({
                    "seshId": sesh.sesh_id,
                    "userId": sesh.user_id,
                    "locationId": sesh.location_id,
                    "notes": sesh.notes,
                    "start": sesh.start,
                    "end": sesh.end,
                    "createdAt": sesh.created_at,
                    "updatedAt": sesh.updated_at,
                })),
            )
                .into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    } else {
        let result = sqlx::query!(
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
        .await;

        match result {
            Ok(sesh) => (
                StatusCode::OK,
                Json(json!({
                    "seshId": sesh.sesh_id,
                    "userId": sesh.user_id,
                    "locationId": sesh.location_id,
                    "notes": sesh.notes,
                    "start": sesh.start,
                    "end": sesh.end,
                    "createdAt": sesh.created_at,
                    "updatedAt": sesh.updated_at,
                })),
            )
                .into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub async fn delete_sesh(
    State(state): State<Arc<AppState>>,
    Path(sesh_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM seshes WHERE sesh_id = $1", sesh_id)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
