use crate::models::{CreateTick, TickSearchParams};
use crate::routes::{ticks_service, AppState};
use axum::extract::{Query, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};

pub async fn create_tick(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTick>,
) -> impl IntoResponse {
    info!("create_tick called with payload: {:?}", payload);

    match ticks_service::create_tick(state, payload).await {
        Ok(tick) => (StatusCode::CREATED, Json(json!(tick))).into_response(),
        Err(error) => {
            error!("Failed to create tick. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn search_ticks(
    State(state): State<Arc<AppState>>,
    Query(params): Query<TickSearchParams>,
) -> impl IntoResponse {
    match ticks_service::get_ticks_by_sesh_id(state, params.sesh_id).await {
        Ok(ticks) => (StatusCode::OK, Json(ticks)).into_response(),
        Err(error) => {
            error!("Failed to get ticks. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
