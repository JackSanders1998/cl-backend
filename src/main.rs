extern crate core;

use std::time::Duration;

use anyhow::Context;
use axum::routing::any;
use axum::{
    routing::{delete, get, post},
    Router,
};
use cl_backend::routes::{
    create_climb, create_location, create_preference, create_sesh, delete_climb, delete_location,
    delete_preference, delete_sesh, get_climb, get_location, get_preference, get_sesh,
    health_check, AppState,
};
use clerk_rs::clerk::Clerk;
use clerk_rs::validators::authorizer::ClerkAuthorizer;
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use http::HeaderMap;
use jsonwebtoken::decode_header;
use shuttle_runtime::SecretStore;
use sqlx::postgres::PgPoolOptions;

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
        .route("/:id", get(get_location))
        .route("/:id", delete(delete_location));

    let preference_routes = Router::new()
        .route("/", post(create_preference))
        .route("/:id", get(get_preference))
        .route("/:id", delete(delete_preference));

    let sesh_routes = Router::new()
        .route("/", post(create_sesh))
        .route("/:id", get(get_sesh))
        .route("/:id", delete(delete_sesh));

    let climb_routes = Router::new()
        .route("/", post(create_climb))
        .route("/:id", get(get_climb))
        .route("/:id", delete(delete_climb));

    let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some(clerk_secret), None);
    let auth = ClerkAuthorizer::new(Clerk::new(config.clone()), true);
    let state = std::sync::Arc::new(AppState {
        db,
        auth,
        clerk: Clerk::new(config.clone()),
    });

    let app = Router::new()
        .route(
            "/",
            any(|header: HeaderMap| async move {
                let decoded_token = decode_header(
                    header
                        .get("authorization")
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .split(" ")
                        .collect::<Vec<_>>()[1],
                );
                format!("Hi {:?}", decoded_token)
            }),
        )
        .route("/healthcheck", get(health_check))
        .nest("/locations", location_routes)
        .nest("/preferences", preference_routes)
        .nest("/seshes", sesh_routes)
        .nest("/climbs", climb_routes)
        .layer(ClerkLayer::new(config, None, true))
        .with_state(state);
    Ok(app.into())
}
