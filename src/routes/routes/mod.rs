use crate::routes::AppState;
use axum::routing::{delete, get, post};
use axum::Router;
use std::sync::Arc;

pub mod routes_controller;
pub mod routes_repository;
pub mod routes_service;

use crate::routes::routes_controller::*;

pub fn routes_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_route))
        .route("/", get(search_routes))
        .route("/:id", get(get_route_by_route_id))
        .route("/:id", delete(delete_route))
}
