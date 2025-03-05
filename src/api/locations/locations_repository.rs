use crate::api::AppState;
use crate::models::{CreateLocation, Location, LocationSearchParams, UpdateLocation};
use sqlx::postgres::PgQueryResult;
use sqlx::Error as PgError;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_location(
    state: Arc<AppState>,
    payload: CreateLocation,
) -> Result<Location, PgError> {
    sqlx::query_as(
        r#"
            INSERT INTO locations (
                name,
                environment,
                author,
                description
            ) VALUES ($1, $2, $3, $4)
            RETURNING *
        "#,
    )
    .bind(payload.name)
    .bind(payload.environment)
    .bind(payload.author)
    .bind(payload.description)
    .fetch_one(&state.db)
    .await
}

pub async fn get_location_by_location_id(
    state: Arc<AppState>,
    location_id: Uuid,
) -> Result<Location, PgError> {
    sqlx::query_as("SELECT * FROM locations WHERE location_id = $1")
        .bind(location_id)
        .fetch_one(&state.db)
        .await
}

pub async fn search_locations(
    state: Arc<AppState>,
    params: LocationSearchParams,
) -> Result<Vec<Location>, PgError> {
    match params.name {
        Some(name) => {
            let formatted_name = "%".to_owned() + &*name + "%";
            sqlx::query_as("SELECT * FROM locations WHERE name LIKE $1")
                .bind(formatted_name)
                .fetch_all(&state.db)
                .await
        }
        None => {
            sqlx::query_as("SELECT * FROM locations")
                .fetch_all(&state.db)
                .await
        }
    }
}

pub async fn update_location_by_location_id(
    state: Arc<AppState>,
    location_id: Uuid,
    payload: UpdateLocation,
) -> Result<Location, PgError> {
    sqlx::query_as(
        r#"
            UPDATE locations
            SET name = COALESCE($1, locations.name),
                environment = COALESCE($2, locations.environment)
           WHERE location_id = $3
           RETURNING *
        "#,
    )
    .bind(payload.name)
    .bind(payload.environment)
    .bind(location_id)
    .fetch_one(&state.db)
    .await
}

pub async fn delete_location_by_location_id(
    state: Arc<AppState>,
    location_id: Uuid,
) -> Result<PgQueryResult, PgError> {
    sqlx::query!("DELETE FROM locations WHERE location_id = $1", location_id)
        .execute(&state.db)
        .await
}
