use crate::api::AppState;
use axum::routing::{delete, get, post};
use axum::Router;
use std::sync::Arc;

pub mod workouts_controller;
pub mod workouts_repository;

use crate::api::workouts_controller::*;

pub fn workouts_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_workout))
        .route("/", get(search_workouts))
        .route("/:id", get(get_workout_by_workout_id))
        .route("/:id", delete(delete_workout))
}
