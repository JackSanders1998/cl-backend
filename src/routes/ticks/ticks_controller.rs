use crate::models::{CreateTick, Tick, TickSearchParams};
use crate::routes::{ticks_repository, AppState};
use axum::extract::{Query, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use tracing::{error, info};

pub async fn create_tick(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTick>,
) -> impl IntoResponse {
    info!("Creating tick with payload: {:?}", payload);

    match ticks_repository::create_tick(state, payload).await {
        Ok(tick) => (
            StatusCode::CREATED,
            Json(Tick {
                tick_id: tick.tick_id,
                sesh_id: tick.sesh_id,
                route_id: tick.route_id,
                discipline: tick.discipline,
                attempt: tick.attempt,
                notes: tick.notes,
                lap_group: tick.lap_group,
                created_at: tick.created_at,
                updated_at: tick.updated_at,
            }),
        )
            .into_response(),
        Err(error) => {
            error!("Failed to create a tick. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn search_ticks(
    State(state): State<Arc<AppState>>,
    Query(params): Query<TickSearchParams>,
) -> impl IntoResponse {
    match ticks_repository::get_tick_and_location_by_sesh_id(state, params).await {
        Ok(ticks) => (StatusCode::OK, Json(ticks)).into_response(),
        Err(error) => {
            error!("Failed to get ticks. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
