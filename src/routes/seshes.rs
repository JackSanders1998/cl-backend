use crate::models::{CreateSesh, Sesh};
use crate::routes::{get_claims, AppState};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use http::HeaderMap;
use serde_json::json;
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
                "sesh_id": sesh.sesh_id,
                "user_id": sesh.user_id,
                "location_id": sesh.location_id,
                "notes": sesh.notes,
                "start": sesh.start,
                "end": sesh.end,
                "created_at": sesh.created_at,
                "updated_at": sesh.updated_at,
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

pub async fn get_active_sesh(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Sesh,
        "SELECT * FROM seshes WHERE seshes.end IS NULL AND user_id = $1 ORDER BY created_at DESC",
        get_claims(headers)
    )
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(sesh) => (StatusCode::OK, Json(sesh)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

// pub async fn update_sesh_by_sesh_id(
//     State(state): State<Arc<AppState>>,
//     Path(sesh_id): Path<Uuid>,
//     Json(payload): Json<UpdateSesh>,
// ) -> impl IntoResponse {
//     if (payload.end_session.is_some()) {
//     //
//     }
//     let result = sqlx::query!(
//         r#"
//             UPDATE seshes
//             SET location_id = COALESCE($1, seshes.location_id),
//                 notes = COALESCE($2, seshes.notes)
//                 end_session = COALESCE($3, seshes.notes)
//            WHERE sesh_id = $4
//            RETURNING *
//         "#,
//         payload.location_id,
//         payload.notes,
//         sesh_id
//     )
//         .fetch_one(&state.db)
//         .await;
//
//     match result {
//         Ok(location) => (
//             StatusCode::OK,
//             Json(json!({
//                "location_id": location.location_id,
//                "user_id":  location.user_id,
//                "name": location.name,
//                "environment": location.environment,
//                "created_at":  location.created_at,
//                "updated_at":  location.updated_at,
//             })),
//         )
//             .into_response(),
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }

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
