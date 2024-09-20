use std::time::Duration;

use anyhow::Context;
use axum::{
    routing::{delete, get, post},
    Router,
};
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use shuttle_runtime::SecretStore;
use sqlx::{postgres::PgPoolOptions};
use cl_backend::routes::{create_location, delete_location, get_location, health_check};

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Get env vars. Exit if any are not found.
    let clerk_secret = secrets
        .get("CLERK_SECRET")
        .context("CLERK_SECRET not found")?;
    let database_url = secrets
        .get("DATABASE_URL")
        .context("DATABASE_URL not found")?;

    // Establish pool connection
    let pool = PgPoolOptions::new()
        .max_connections(64)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("PG cannot start. Exiting.");

    let location_routes = Router::new()
        .route("/", post(create_location))
        .route("/:id", get(get_location))
        .route("/:id", delete(delete_location));

    let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some(clerk_secret), None);

    let app = Router::new()
        .route("/healthcheck", get(health_check))
        .nest("/locations", location_routes)
        .layer(ClerkLayer::new(config, None, true))
        .with_state(pool);
    Ok(app.into())
}
