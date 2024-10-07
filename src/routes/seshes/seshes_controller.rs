pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{CreateSesh, Sesh, SeshSearchParams, UpdateSesh};
use crate::routes::{get_claims, seshes_repository, seshes_service, AppState};
use axum::extract::{Path, Query, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use http::HeaderMap;
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/seshes",
    request_body = CreateSesh,
    responses(
        (status = 201, description = "Create a sesh", body = Sesh),
        (status = 500, description = "Sesh was not created")
    )
)]
pub async fn create_sesh(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSesh>,
) -> impl IntoResponse {
    let result = seshes_repository::create_sesh(state, payload, get_claims(headers)).await;

    match result {
        Ok(sesh) => (
            StatusCode::CREATED,
            Json(Sesh {
                sesh_id: sesh.sesh_id,
                user_id: sesh.user_id,
                location_id: sesh.location_id,
                notes: sesh.notes,
                start: sesh.start,
                end: sesh.end,
                created_at: sesh.created_at,
                updated_at: sesh.updated_at,
            }),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/seshes/{sesh_id}",
    params(
        ("sesh_id", description = "sesh id"),
    ),
    responses(
        (status = 200, description = "Get a sesh successfully", body = Sesh),
        (status = 404, description = "Sesh was not found")
    )
)]
pub async fn get_sesh_by_sesh_id(
    State(state): State<Arc<AppState>>,
    Path(sesh_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = seshes_repository::get_sesh_by_sesh_id(state, sesh_id).await;

    match result {
        Ok(sesh) => (StatusCode::OK, Json(sesh)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/seshes",
    params(SeshSearchParams),
    responses(
        (status = 200, description = "Get sesh(es) successfully", body = [Sesh]),
        (status = 404, description = "No sesh found")
    )
)]
pub async fn search_seshes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SeshSearchParams>,
) -> impl IntoResponse {
    let result = seshes_repository::search_seshes(state, params).await;

    match result {
        Ok(seshes) => (StatusCode::OK, Json(seshes)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/seshes/active",
    responses(
        (status = 200, description = "Get active sesh successfully", body = [SeshWithLocationAndClimbs]),
        (status = 404, description = "No active sesh found")
    )
)]
pub async fn get_active_sesh(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let seshes = seshes_repository::get_all_active_sesh_data(state, get_claims(headers)).await;

    match seshes {
        Ok(seshes) => {
            let sesh_with_location_and_climbs = seshes_service::map_db_rows_to_sesh_object(seshes);
            match sesh_with_location_and_climbs {
                Ok(sesh) => (StatusCode::OK, Json(sesh)).into_response(),
                _ => StatusCode::NOT_FOUND.into_response(),
            }
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/seshes/{sesh_id}",
    params(
        ("sesh_id", description = "sesh id"),
    ),
    request_body = UpdateSesh,
    responses(
        (status = 200, description = "Update sesh successfully", body = Sesh),
        (status = 500, description = "Sesh was not updated")
    )
)]
pub async fn update_sesh_by_sesh_id(
    State(state): State<Arc<AppState>>,
    Path(sesh_id): Path<Uuid>,
    Json(payload): Json<UpdateSesh>,
) -> impl IntoResponse {
    let result = seshes_repository::update_sesh_by_sesh_id(state, sesh_id, payload).await;

    match result {
        Ok(sesh) => (
            StatusCode::OK,
            Json(Sesh {
                sesh_id: sesh.sesh_id,
                user_id: sesh.user_id,
                location_id: sesh.location_id,
                notes: sesh.notes,
                start: sesh.start,
                end: sesh.end,
                created_at: sesh.created_at,
                updated_at: sesh.updated_at,
            }),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/seshes/{sesh_id}",
    params(
        ("sesh_id", description = "sesh id"),
    ),
    responses(
        (status = 204, description = "Delete a sesh"),
        (status = 500, description = "Sesh was not deleted")
    )
)]
pub async fn delete_sesh(
    State(state): State<Arc<AppState>>,
    Path(sesh_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = seshes_repository::delete_sesh(state, sesh_id).await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
