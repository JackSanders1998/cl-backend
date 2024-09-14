use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use rspc::{Config, Router as RspcRouter};

fn router() -> Arc<RspcRouter> {
    <RspcRouter>::new()
        .config(Config::new().export_ts_bindings("./cl-bindings/index.d.ts"))
        .query("version", |t| t(|_ctx, _input: ()| "1.0.0"))
        .build()
        .arced()
}

async fn post_log(
    State(state): State<MyState>,
    Json(data): Json<LogNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Log>(
        "INSERT INTO logs (grade, metadata) VALUES ($1, $2) RETURNING id, grade, metadata",
    )
    .bind(&data.grade)
    .bind(&data.metadata)
    .fetch_one(&state.pool)
    .await
    {
        Ok(log) => Ok((StatusCode::CREATED, Json(log))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn get_log(State(state): State<MyState>) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Log>("SELECT * FROM logs")
        .fetch_all(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[shuttle_runtime::main]
async fn main(
    // #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // Get env vars. Exit if any are not found.
    let clerk_secret = secrets
        .get("CLERK_SECRET")
        .context("CLERK_SECRET not found")?;
    let database_url = secrets
        .get("DATABASE_URL")
        .context("DATABASE_URL not found")?;

    // Establish pool connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("PG cannot start. Exiting.");

    // let cors = CorsLayer::new()
    //     .allow_methods([Method::GET, Method::POST])
    //     .allow_origin(Any)
    //     .allow_headers([CONTENT_TYPE]);

    // Run outstanding db migrations.
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed. Exiting.");

    let state = MyState { pool };

    let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some(clerk_secret), None);
    let app = Router::new()
        .route("/log", post(post_log))
        .route("/log", get(get_log))
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .nest("/rspc", rspc_axum::endpoint(router(), || ()))
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
