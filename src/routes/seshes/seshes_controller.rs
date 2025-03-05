use crate::models::{CreateSesh, Location, SeshWithLocation, UpdateSesh};
use crate::routes::{get_claims, seshes_repository, seshes_service, AppState};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use http::HeaderMap;
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

pub async fn create_sesh(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSesh>,
) -> impl IntoResponse {
    info!("create_sesh called with payload: {:?}", payload);

    match seshes_service::create_sesh(state, get_claims(headers), payload).await {
        Ok(sesh) => (StatusCode::CREATED, Json(json!(sesh))).into_response(),
        Err(error) => {
            error!("Failed to create sesh. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_sesh_by_sesh_id(
    State(state): State<Arc<AppState>>,
    Path(sesh_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("get_sesh_by_sesh_id called with sesh_id: {:?}", sesh_id);

    match seshes_service::get_sesh_by_sesh_id(state, sesh_id).await {
        Ok(seshes) => (StatusCode::OK, Json(seshes)).into_response(),
        Err(error) => {
            error!("Failed to get sesh by id. Error: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn search_seshes(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match seshes_service::search_seshes(state, get_claims(headers)).await {
        Ok(seshes) => (StatusCode::OK, Json(seshes)).into_response(),
        Err(error) => {
            error!("Failed to get search seshes. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_active_sesh(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user_id = get_claims(headers);
    match seshes_repository::get_active_seshes(state.clone(), user_id.clone()).await {
        Ok(active_seshes) => {
            if active_seshes.len() > 1 {
                info!("More than one active sesh found. Reconciling...");
                let _ = seshes_repository::reconcile_active_seshes(state, user_id).await;
            };
            if active_seshes.len() == 0 {
                return (StatusCode::OK, Json(())).into_response();
            }
            let sesh = active_seshes.into_iter().nth(0).unwrap();
            let active_sesh = SeshWithLocation {
                sesh_id: sesh.sesh_id,
                user_id: sesh.user_id,
                notes: sesh.notes,
                start: sesh.start,
                end: sesh.end,
                created_at: sesh.created_at,
                updated_at: sesh.updated_at,
                location: Location {
                    location_id: sesh.location_id,
                    author: sesh.author,
                    name: sesh.name,
                    environment: sesh.environment,
                    description: sesh.description,
                    created_at: sesh.location_created_at,
                    updated_at: sesh.location_updated_at,
                },
            };

            (StatusCode::OK, Json(active_sesh)).into_response()
        }
        Err(error) => {
            error!("Failed to get an active sesh. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_sesh_by_sesh_id(
    State(state): State<Arc<AppState>>,
    Path(sesh_id): Path<Uuid>,
    Json(payload): Json<UpdateSesh>,
) -> impl IntoResponse {
    match seshes_repository::update_sesh_by_sesh_id(state, sesh_id, payload).await {
        Ok(sesh) => (StatusCode::OK, Json(sesh)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

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
