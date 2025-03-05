use crate::api::AppState;
use axum::routing::{delete, get, post};
use axum::Router;
use std::sync::Arc;

pub mod preferences_controller;
pub mod preferences_repository;

pub use preferences_controller::*;

pub fn preferences_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_preference))
        .route("/:id", get(get_preference_by_preference_id))
        .route("/", get(get_preference_by_user_id))
        .route("/:id", delete(delete_preference))
}
