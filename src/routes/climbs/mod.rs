use crate::routes::AppState;
use axum::routing::{delete, get, post};
use axum::Router;
use std::sync::Arc;

pub mod climbs_controller;
pub mod climbs_repository;

use crate::routes::climbs_controller::*;

pub fn climbs_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_climb))
        .route("/", get(search_climbs))
        .route("/:id", get(get_climb_by_climb_id))
        .route("/:id", delete(delete_climb))
}
