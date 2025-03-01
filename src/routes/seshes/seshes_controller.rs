use crate::models::CreateSesh;
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
    match seshes_service::get_active_sesh(state, get_claims(headers)).await {
        Ok(active_sesh) => (StatusCode::OK, Json(active_sesh)).into_response(),
        Err(error) => {
            error!("Failed to get an active sesh. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// pub async fn update_sesh_by_sesh_id(
//     State(state): State<Arc<AppState>>,
//     Path(sesh_id): Path<Uuid>,
//     Json(payload): Json<UpdateSesh>,
// ) -> impl IntoResponse {
//     match seshes_repository::update_sesh_by_sesh_id(state.clone(), sesh_id, payload).await {
//         Ok(sesh) => match seshes_service::get_hydrated_seshes(state, vec![sesh.sesh_id]).await {
//             Ok(seshes) => (StatusCode::OK, Json(seshes)).into_response(),
//             Err(_) => StatusCode::NOT_FOUND.into_response(),
//         },
//         Err(error) => {
//             error!("Failed to get delete sesh. Error: {:?}", error);
//             StatusCode::NOT_FOUND.into_response()
//         }
//     }
// }

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
