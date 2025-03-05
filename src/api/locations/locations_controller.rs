use crate::api::{locations_repository, AppState};
use crate::models::{CreateLocation, Location, LocationSearchParams, UpdateLocation};
use axum::extract::{Path, Query, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/locations",
    request_body = CreateLocation,
    responses(
        (status = 201, description = "Create a location", body = Location),
        (status = 500, description = "Location was not created")
    )
)]
pub async fn create_location(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateLocation>,
) -> impl IntoResponse {
    info!("Creating location with payload: {:?}", payload);
    let result = locations_repository::create_location(state, payload).await;

    match result {
        Ok(location) => (
            StatusCode::CREATED,
            Json(Location {
                location_id: location.location_id,
                author: location.author,
                name: location.name,
                environment: location.environment,
                description: location.description,
                created_at: location.created_at,
                updated_at: location.updated_at,
            }),
        )
            .into_response(),
        Err(error) => {
            error!("Failed to get an active sesh. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/locations/{location_id}",
    params(
        ("location_id", description = "location id"),
    ),
    responses(
        (status = 200, description = "Get a location successfully", body = Location),
        (status = 404, description = "Location was not found")
    )
)]
pub async fn get_location_by_location_id(
    State(state): State<Arc<AppState>>,
    Path(location_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = locations_repository::get_location_by_location_id(state, location_id).await;

    match result {
        Ok(location) => (StatusCode::OK, Json(location)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/locations",
    params(LocationSearchParams),
    responses(
        (status = 200, description = "Get location(s) successfully", body = [Location]),
        (status = 404, description = "No location found")
    )
)]
pub async fn search_locations(
    State(state): State<Arc<AppState>>,
    Query(params): Query<LocationSearchParams>,
) -> impl IntoResponse {
    let result = locations_repository::search_locations(state, params).await;

    match result {
        Ok(locations) => (StatusCode::OK, Json(locations)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/locations/{location_id}",
    params(
        ("location_id", description = "location id"),
    ),
    request_body = UpdateLocation,
    responses(
        (status = 200, description = "Update location successfully", body = Location),
        (status = 500, description = "Location was not updated")
    )
)]
pub async fn update_location_by_location_id(
    State(state): State<Arc<AppState>>,
    Path(location_id): Path<Uuid>,
    Json(payload): Json<UpdateLocation>,
) -> impl IntoResponse {
    let result =
        locations_repository::update_location_by_location_id(state, location_id, payload).await;

    match result {
        Ok(location) => (
            StatusCode::OK,
            Json(Location {
                location_id: location.location_id,
                author: location.author,
                name: location.name,
                environment: location.environment,
                description: location.description,
                created_at: location.created_at,
                updated_at: location.updated_at,
            }),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/locations/{location_id}",
    params(
        ("location_id", description = "location id"),
    ),
    responses(
        (status = 204, description = "Delete a location"),
        (status = 500, description = "Location was not deleted")
    )
)]
pub async fn delete_location_by_location_id(
    State(state): State<Arc<AppState>>,
    Path(location_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = locations_repository::delete_location_by_location_id(state, location_id).await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
