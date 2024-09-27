extern crate core;

use std::time::Duration;

use anyhow::Context;
use axum::{
    routing::{delete, get, post},
    Router,
};
use cl_backend::routes::{create_climb, create_location, create_preference, create_sesh, delete_climb, delete_location_by_location_id, delete_preference, delete_sesh, get_climb_by_climb_id, get_location_by_location_id, get_preference_by_preference_id, get_sesh, health_check, search_locations, AppState, get_preference_by_user_id};
use clerk_rs::clerk::Clerk;
use clerk_rs::validators::authorizer::ClerkAuthorizer;
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use shuttle_runtime::SecretStore;
use sqlx::postgres::{PgPoolOptions};

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
    let db = PgPoolOptions::new()
        .max_connections(64)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("PG cannot start. Exiting.");

    let location_routes = Router::new()
        .route("/", post(create_location))
        .route("/:id", get(get_location_by_location_id))
        .route("/", get(search_locations))
        .route("/:id", delete(delete_location_by_location_id));

    let preference_routes = Router::new()
        .route("/", post(create_preference))
        .route("/:id", get(get_preference_by_preference_id))
        .route("/", get(get_preference_by_user_id))
        .route("/:id", delete(delete_preference));

    let sesh_routes = Router::new()
        .route("/", post(create_sesh))
        .route("/:id", get(get_sesh))
        .route("/:id", delete(delete_sesh));

    let climb_routes = Router::new()
        .route("/", post(create_climb))
        .route("/:id", get(get_climb_by_climb_id))
        .route("/:id", delete(delete_climb));

    let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some(clerk_secret), None);
    let auth = ClerkAuthorizer::new(Clerk::new(config.clone()), true);
    let state = std::sync::Arc::new(AppState {
        db,
        auth,
        clerk: Clerk::new(config.clone()),
    });

    let app = Router::new()
        .route("/healthcheck", get(health_check))
        .nest("/locations", location_routes)
        .nest("/preferences", preference_routes)
        .nest("/seshes", sesh_routes)
        .nest("/climbs", climb_routes)
        .layer(ClerkLayer::new(config, None, true))
        .with_state(state);
    Ok(app.into())
}
