use std::sync::Arc;

use crate::database::Database;
use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use cl_backend::Database;
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use rspc::Router as RspcRouter;
use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;
use specta::Type;
use sqlx::FromRow;
use tracing::info;

fn router() -> Arc<RspcRouter> {
    <RspcRouter>::new()
        .query("version", |t| t(|_ctx, _input: ()| "1.0.0"))
        .build()
        .arced()
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Get env vars. Exit if any are not found.
    let clerk_secret = secrets
        .get("CLERK_SECRET")
        .context("CLERK_SECRET not found")?;
    let database_url = secrets
        .get("DATABASE_URL")
        .context("DATABASE_URL not found")?;

    let db = Database::connect(&database_url, false)
        .await
        .expect("could not initialize the database connection pool");

    // let user_routes = Router::new().route("/:id", get(|| async {}));
    // let log_routes = Router::new()
    //     .route("/", post(post_log))
    //     .route("/", patch(patch_log))
    //     .route("/", get(get_log))
    //     .route("/", delete(delete_log));

    let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some(clerk_secret), None);
    let app = Router::new()
        .nest("/", rspc_axum::endpoint(router(), || ()))
        // .nest("/log", log_routes)
        .layer(ClerkLayer::new(config, None, true))
        .with_state(db);
    Ok(app.into())
}

#[derive(Serialize, FromRow, Type)]
struct Log {
    pub id: i32,
    pub grade: String,
    pub metadata: String,
}

#[derive(Deserialize, Type)]
pub struct LogNew {
    pub grade: String,
    pub metadata: String,
}

#[derive(Type)]
pub struct MyCustomType {
    pub my_field: String,
}

#[cfg(test)]
mod tests {
    use crate::MyCustomType;
    use specta::{ts::*, *};

    #[test]
    fn test_rspc_router() {
        super::router();
    }

    #[test]
    fn test_specta() {
        assert_eq!(
            ts::export::<MyCustomType>(&ExportConfiguration::default()).unwrap(),
            "export type MyCustomType = { my_field: string }".to_string()
        );
    }
}
