pub mod analytics_controller;
pub mod analytics_repository;

use crate::routes::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub use analytics_controller::*;

pub fn analytics_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_analytics))
}