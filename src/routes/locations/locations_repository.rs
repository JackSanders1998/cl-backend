use crate::models::{CreateLocation, Location, LocationSearchParams, UpdateLocation};
use crate::routes::AppState;
use sqlx::postgres::PgQueryResult;
use sqlx::Error as PgError;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_location(
    state: Arc<AppState>,
    payload: CreateLocation,
    user_id: String,
) -> Result<Location, PgError> {
    sqlx::query_as(
        r#"
            INSERT INTO locations (
                user_id,
                name,
                environment
            ) VALUES ($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(payload.name)
    .bind(payload.environment)
    .fetch_one(&state.db)
    .await
}

pub async fn get_location_by_location_id(
    state: Arc<AppState>,
    location_id: Uuid,
) -> Result<Location, PgError> {
    sqlx::query_as!(
        Location,
        "SELECT * FROM locations WHERE location_id = $1",
        location_id
    )
    .fetch_one(&state.db)
    .await
}

pub async fn search_locations(
    state: Arc<AppState>,
    params: LocationSearchParams,
    user_id: String,
) -> Result<Vec<Location>, PgError> {
    match params.name {
        Some(name) => {
            let formatted_name = "%".to_owned() + &*name + "%";
            sqlx::query_as!(
                Location,
                "SELECT * FROM locations WHERE user_id = $1 AND name LIKE $2",
                user_id,
                formatted_name
            )
            .fetch_all(&state.db)
            .await
        }
        None => {
            sqlx::query_as!(Location, "SELECT * FROM locations WHERE user_id = $1", user_id)
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
    sqlx::query_as!(
        Location,
        r#"
            UPDATE locations
            SET name = COALESCE($1, locations.name),
                environment = COALESCE($2, locations.environment)
           WHERE location_id = $3
           RETURNING *
        "#,
        payload.name,
        payload.environment,
        location_id
    )
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
