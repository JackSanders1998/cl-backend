extern crate core;

use std::time::Duration;

use anyhow::Context;
use axum::routing::get;
use axum::Router;
use cl_backend::api::{
    health_check, locations_router, preferences_router, routes_router, seshes_router, ticks_router,
    AppState,
};
use cl_backend::utils::CustomTraceLayer;
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use shuttle_runtime::SecretStore;
use sqlx::postgres::PgPoolOptions;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token_jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

#[utoipauto(paths = "./src/api from cl_backend::api, ./src/models from cl_backend::models")]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Climbing Logger", description = "Climbing logger endpoints.")
    ),
    modifiers(&SecurityAddon),
    security(
        ("token_jwt" = ["general"]),
    )
)]
pub struct ApiDoc;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Initialize trace layer
    CustomTraceLayer::init();

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
        .acquire_slow_threshold(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("PG cannot start. Exiting.");

    let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some(clerk_secret), None);
    let state = std::sync::Arc::new(AppState { db });

    let app = Router::new()
        .route("/healthcheck", get(health_check))
        .nest("/locations", locations_router())
        .nest("/preferences", preferences_router())
        .nest("/seshes", seshes_router())
        .nest("/routes", routes_router())
        .nest("/ticks", ticks_router())
        .layer(ClerkLayer::new(config, None, true))
        .layer(CustomTraceLayer::setup())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    Ok(app.into())
}

// TODO: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
