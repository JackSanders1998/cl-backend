use crate::api::{routes_repository, AppState};
use crate::models::{CreateRoute, Route};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

use super::routes_service;

#[utoipa::path(
    post,
    path = "/routes",
    request_body = CreateRoute,
    responses(
        (status = 201, description = "Create a route", body = Route),
        (status = 500, description = "Route was not created")
    )
)]
pub async fn create_route(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRoute>,
) -> impl IntoResponse {
    let result = routes_repository::create_route(state, payload).await;

    match result {
        Ok(route) => (
            StatusCode::CREATED,
            Json(Route {
                route_id: route.route_id,
                location_id: route.location_id,
                grade: route.grade,
                scale: route.scale,
                disciplines: route.disciplines,
                author: route.author,
                description: route.description,
                created_at: route.created_at,
                updated_at: route.updated_at,
            }),
        )
            .into_response(),
        Err(error) => {
            error!("Failed to create route. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/routes/{route_id}",
    params(
        ("route_id", description = "route id"),
    ),
    responses(
        (status = 200, description = "Get a route successfully", body = RouteWithLocation),
        (status = 404, description = "Route was not found")
    )
)]
pub async fn get_route_by_route_id(
    State(state): State<Arc<AppState>>,
    Path(route_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("get_route_by_route_id called with: {:?}", route_id);
    let result = routes_service::get_route_by_route_id(state, route_id).await;
    info!("Result: {:?}", result);

    match result {
        Ok(route) => (StatusCode::OK, Json(route)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/routes",
    responses(
        (status = 200, description = "Get route(s) successfully", body = [Route]),
        (status = 404, description = "No route found")
    )
)]
pub async fn search_routes(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let result = routes_repository::get_all_routes(state).await;

    match result {
        Ok(routes) => (StatusCode::OK, Json(routes)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/routes/{route_id}",
    params(
        ("route_id", description = "route id"),
    ),
    responses(
        (status = 204, description = "Delete a route"),
        (status = 500, description = "Route was not deleted")
    )
)]
pub async fn delete_route(
    State(state): State<Arc<AppState>>,
    Path(route_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = routes_repository::delete_route(state, route_id).await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
