extern crate core;

use std::fs::File;
use std::io::Write;
use std::time::Duration;

use anyhow::Context;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use cl_backend::routes::{
    create_climb, create_location, create_preference, create_sesh, delete_climb,
    delete_location_by_location_id, delete_preference, delete_sesh, get_active_sesh,
    get_climb_by_climb_id, get_location_by_location_id, get_preference_by_preference_id,
    get_preference_by_user_id, get_sesh_by_sesh_id, health_check, search_climbs, search_locations,
    search_seshes, update_location_by_location_id, update_sesh_by_sesh_id, AppState,
};
use cl_backend::utils::CustomTraceLayer;
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use shuttle_runtime::SecretStore;
use sqlx::postgres::PgPoolOptions;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

#[utoipauto(paths = "./src/routes from cl_backend::routes, ./src/models from cl_backend::models")]
#[derive(OpenApi)]
#[openapi(
tags(
(name = "Climbing Logger", description = "Climbing logger endpoints.")
),
modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "api_key1",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
            schema.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("cl_apikey"))),
            )
        }
    }
}

fn generate_open_api_bindings() -> std::io::Result<()> {
    let mut file = File::create("./cl-bindings/api.json")?;
    let json = ApiDoc::openapi().to_pretty_json()?;
    println!("{}", &json);
    file.write_all(json.as_bytes())
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Initialize trace layer
    CustomTraceLayer::init();
    // Generate openapi bindings
    let _ = generate_open_api_bindings();

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
        .route("/:id", patch(update_location_by_location_id))
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
        .route("/:id", patch(update_sesh_by_sesh_id))
        .route("/", get(search_seshes))
        .route("/:id", get(get_sesh_by_sesh_id))
        .route("/active", get(get_active_sesh))
        .route("/:id", delete(delete_sesh));

    let climb_routes = Router::new()
        .route("/", post(create_climb))
        .route("/", get(search_climbs))
        .route("/:id", get(get_climb_by_climb_id))
        .route("/:id", delete(delete_climb));

    let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some(clerk_secret), None);
    let state = std::sync::Arc::new(AppState { db });

    let app = Router::new()
        .route("/healthcheck", get(health_check))
        .nest("/locations", location_routes)
        .nest("/preferences", preference_routes)
        .nest("/seshes", sesh_routes)
        .nest("/climbs", climb_routes)
        .layer(ClerkLayer::new(config, None, true))
        .layer(CustomTraceLayer::new())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);
    Ok(app.into())
}

// TODO: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
