use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use rspc::Router as RspcRouter;
use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;
use specta::Type;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};

pub struct LogController;

impl LogController {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::get_logs))
            .route("/", post(Self::create_log()))
    }

    pub async fn create_log(
        State(state): State<MyState>,
        Json(data): Json<LogNew>,
    ) -> Result<Vec<Log>> {
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

    pub async fn get_logs(
        State(state): State<MyState>,
    ) -> Result<impl IntoResponse, impl IntoResponse> {
        match sqlx::query_as::<_, Log>("SELECT * FROM logs")
            .fetch_all(&state.pool)
            .await
        {
            Ok(todo) => Ok((StatusCode::OK, Json(todo))),
            Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
        }
    }
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

#[derive(Clone)]
struct MyState {
    pool: PgPool,
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
