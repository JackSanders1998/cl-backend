use crate::routes::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub mod ticks_controller;
pub mod ticks_repository;
pub mod ticks_service;

use crate::routes::ticks::ticks_controller::*;

pub fn ticks_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_tick))
        .route("/", get(search_ticks))
}
