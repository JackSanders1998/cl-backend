use crate::api::{routes_repository, AppState};
use crate::models::{CreateRoute, Location, Route, RouteWithLocation};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use std::io::ErrorKind;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

pub async fn get_route_by_route_id(
    state: Arc<AppState>,
    route_id: Uuid,
) -> Result<RouteWithLocation, ErrorKind> {
    match routes_repository::get_route_and_location_by_route_id(state, route_id).await {
        Ok(route) => Ok(RouteWithLocation {
            route_id: route.route_id,
            grade: route.grade,
            scale: route.scale,
            disciplines: route.disciplines,
            author: route.author,
            description: route.description,
            location: Location {
                location_id: route.location_id,
                author: route.location_author,
                name: route.name,
                environment: route.environment,
                description: route.location_description,
                created_at: route.location_created_at,
                updated_at: route.location_updated_at,
            },
            created_at: route.created_at,
            updated_at: route.updated_at,
        }),
        Err(error) => {
            error!("Failed to get route by route_id. Error: {:?}", error);
            Err(ErrorKind::NotFound)
        }
    }
}

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

pub async fn search_routes(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let result = routes_repository::get_all_routes(state).await;

    match result {
        Ok(routes) => (StatusCode::OK, Json(routes)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

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
