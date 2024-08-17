use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use serde::{Serialize, Deserialize};
use shuttle_runtime::SecretStore;
use sqlx::{FromRow, PgPool};


async fn post_log(
    State(state): State<MyState>,
    Json(data): Json<LogNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Log>("INSERT INTO logs (grade, metadata) VALUES ($1, $2) RETURNING id, grade, metadata")
        .bind(&data.grade)
        .bind(&data.metadata)
        .fetch_one(&state.pool)
        .await
    {
        Ok(log) => Ok((StatusCode::CREATED, Json(log))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
async fn patch_log() -> &'static str {
    "Patch Log"
}
async fn get_log(
    Path(id): Path<i32>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Log>("SELECT * FROM logs")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
async fn delete_log() -> &'static str {
    "Delete Log"
}
async fn hello_world() -> &'static str {
    "Hello world!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed :(");

    let state = MyState { pool };

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
        .layer(ClerkLayer::new(config, None, true))
        .with_state(state);
    Ok(app.into())
}

#[derive(Serialize, FromRow)]
struct Log {
    pub id: i32,
    pub grade: String,
    pub metadata: String,
}

#[derive(Deserialize)]
struct LogNew {
    pub grade: String,
    pub metadata: String,
}

#[derive(Clone)]
struct MyState {
    pool: PgPool,
}
