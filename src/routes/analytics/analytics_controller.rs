use crate::models::{CreateLocation, Location, LocationSearchParams, UpdateLocation};
use crate::routes::{get_claims, locations_repository, AppState};
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/locations",
    params(LocationSearchParams),
    responses(
        (status = 200, description = "Get location(s) successfully", body = [Location]),
        (status = 404, description = "No location found")
    )
)]
pub async fn get_analytics(
    State(state): State<Arc<AppState>>,
    Query(params): Query<LocationSearchParams>,
) -> impl IntoResponse {
    let result = locations_repository::search_locations(state, params).await;

    match result {
        Ok(locations) => (StatusCode::OK, Json(locations)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
