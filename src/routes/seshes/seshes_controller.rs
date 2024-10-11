pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{CreateSesh, SeshSearchParams, UpdateSesh};
use crate::routes::{get_claims, seshes_repository, seshes_service, AppState};
use axum::extract::{Path, Query, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use http::HeaderMap;
use serde_json::json;
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
    match seshes_repository::create_sesh(state.clone(), payload, get_claims(headers)).await {
        Ok(sesh) => {
            match seshes_service::get_seshes_with_location_and_climbs(state, vec![sesh.sesh_id])
                .await
            {
                Ok(seshes) => {
                    if seshes.len() == 1 {
                        (StatusCode::CREATED, Json(json!(seshes.into_iter().nth(0))))
                            .into_response()
                    } else {
                        StatusCode::NOT_FOUND.into_response()
                    }
                }
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
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
    match seshes_service::get_seshes_with_location_and_climbs(state, vec![sesh_id]).await {
        Ok(seshes) => {
            if seshes.len() == 1 {
                (StatusCode::CREATED, Json(json!(seshes.into_iter().nth(0)))).into_response()
            } else {
                StatusCode::NOT_FOUND.into_response()
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
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
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Query(params): Query<SeshSearchParams>,
) -> impl IntoResponse {
    match seshes_repository::search_seshes(state.clone(), params, get_claims(headers)).await {
        Ok(sesh_ids) => {
            match seshes_service::get_seshes_with_location_and_climbs(
                state,
                seshes_service::get_ids_from_struct(sesh_ids),
            )
            .await
            {
                Ok(seshes) => (StatusCode::OK, Json(seshes)).into_response(),
                Err(_) => StatusCode::NOT_FOUND.into_response(),
            }
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/seshes/active",
    responses(
        (status = 200, description = "Get active sesh successfully", body = [Sesh]),
        (status = 404, description = "No active sesh found")
    )
)]
pub async fn get_active_sesh(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match seshes_repository::get_all_active_sesh_data(state, get_claims(headers)).await {
        Ok(seshes) => {
            // If there are no rows in the db, return empty
            if seshes.len() == 0 {
                return StatusCode::OK.into_response();
            }
            // Else, continue with mapping
            match seshes_service::map_db_rows_to_sesh_object(seshes) {
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
    match seshes_repository::update_sesh_by_sesh_id(state.clone(), sesh_id, payload).await {
        Ok(sesh) => {
            match seshes_service::get_seshes_with_location_and_climbs(state, vec![sesh.sesh_id])
                .await
            {
                Ok(seshes) => (StatusCode::OK, Json(seshes)).into_response(),
                Err(_) => StatusCode::NOT_FOUND.into_response(),
            }
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
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
