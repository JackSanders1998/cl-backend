use anyhow::Context;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use shuttle_runtime::SecretStore;

async fn post_log() -> &'static str {
    "Post Log"
}
async fn patch_log() -> &'static str {
    "Patch Log"
}
async fn get_log() -> &'static str {
    "Get Log"
}

async fn delete_log() -> &'static str {
    "Delete Log"
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

// #[tokio::main]
#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let clerk_secret = secrets
        .get("CLERK_SECRET")
        .context("CLERK_SECRET not found")?;

    let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some(clerk_secret), None);
    let app = Router::new()
        .route("/log", post(post_log))
        .route("/log", patch(patch_log))
        .route("/log", get(get_log))
        .route("/log", delete(delete_log))
        .route("/hello-world", get(hello_world))
        .layer(ClerkLayer::new(config, None, true));
    Ok(app.into())
}
