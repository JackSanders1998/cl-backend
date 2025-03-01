use crate::routes::AppState;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use std::sync::Arc;

pub mod seshes_controller;
pub mod seshes_repository;
pub mod seshes_service;

use crate::routes::seshes_controller::*;

pub fn seshes_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_sesh))
        .route("/:id", patch(update_sesh_by_sesh_id))
        .route("/", get(search_seshes))
        .route("/:id", get(get_sesh_by_sesh_id))
        .route("/active", get(get_active_sesh))
        .route("/:id", delete(delete_sesh))
}
