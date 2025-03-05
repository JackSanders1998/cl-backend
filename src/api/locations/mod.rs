pub mod locations_controller;
pub mod locations_repository;

use crate::api::AppState;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use std::sync::Arc;

pub use locations_controller::*;

pub fn locations_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_location))
        .route("/:id", patch(update_location_by_location_id))
        .route("/:id", get(get_location_by_location_id))
        .route("/", get(search_locations))
        .route("/:id", delete(delete_location_by_location_id))
}
