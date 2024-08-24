mod logs;

use axum::routing::*;

pub async fn health() -> &'static str {
    "🚀🚀🚀 Server Running"
}

pub fn app() -> Router {
    Router::new()
        .nest("/users", logs)
        .nest("/categories", CategoryController::app())
        .route("/health", get(health))
}
