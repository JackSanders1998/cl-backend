use crate::models::{CreateRoute, Discipline, Route, Scale};
use crate::routes::AppState;
use sqlx::postgres::PgQueryResult;
use sqlx::Error as PgError;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_route(state: Arc<AppState>, payload: CreateRoute) -> Result<Route, PgError> {
    sqlx::query_as(
        r#"
            INSERT INTO routes (
                location_id,
                grade,
                scale,
                disciplines,
                author,
                description
            ) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
        "#,
    )
    .bind(payload.location_id)
    .bind(payload.grade)
    .bind(payload.scale)
    .bind(payload.disciplines)
    .bind(payload.author)
    .bind(payload.description)
    .fetch_one(&state.db)
    .await
}

pub async fn get_route_by_route_id(state: Arc<AppState>, route_id: Uuid) -> Result<Route, PgError> {
    sqlx::query_as!(
        Route,
        r#"
            SELECT
                route_id,
                location_id,
                grade,
                scale as "scale: Scale",
                disciplines as "disciplines: Vec<Discipline>",
                author,
                description,
                created_at,
                updated_at
            FROM routes
            WHERE route_id = $1
        "#,
        route_id
    )
    .fetch_one(&state.db)
    .await
}

pub async fn get_all_routes(state: Arc<AppState>) -> Result<Vec<Route>, PgError> {
    sqlx::query_as!(
        Route,
        r#"
            SELECT
                route_id,
                location_id,
                grade,
                scale as "scale: Scale",
                disciplines as "disciplines: Vec<Discipline>",
                author,
                description,
                created_at,
                updated_at
            FROM routes
        "#
    )
    .fetch_all(&state.db)
    .await
}

pub async fn delete_route(state: Arc<AppState>, route_id: Uuid) -> Result<PgQueryResult, PgError> {
    sqlx::query!("DELETE FROM routes WHERE route_id = $1", route_id)
        .execute(&state.db)
        .await
}
