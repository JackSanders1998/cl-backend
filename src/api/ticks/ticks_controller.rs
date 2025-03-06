use crate::api::{ticks_service, AppState};
use crate::models::{CreateTick, TickSearchParams};
use axum::extract::{Query, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};

#[utoipa::path(
    post,
    path = "/ticks",
    request_body = CreateTick,
    responses(
        (status = 201, description = "Create a tick", body = CreateTick),
        (status = 500, description = "Tick was not created")
    )
)]
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

#[utoipa::path(
    get,
    path = "/ticks",
    params(TickSearchParams),
    responses(
        (status = 200, description = "Get tick(s) successfully", body = [TickWithRoute]),
        (status = 404, description = "No tick found")
    )
)]
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
