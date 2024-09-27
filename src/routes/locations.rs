use crate::models::{CreateLocation, Location, UpdateLocation};
use crate::routes::{get_claims, AppState};
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_location(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateLocation>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO locations (user_id, name, environment) VALUES ($1, $2, $3) RETURNING *",
        get_claims(headers),
        payload.name,
        payload.environment
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(location) => (
            StatusCode::CREATED,
            Json(json!({
               "location_id": location.location_id,
               "user_id":  location.user_id,
                "name": location.name,
                "environment": location.environment,
                "created_at":  location.created_at,
                "updated_at":  location.updated_at,
            })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_location_by_location_id(
    State(state): State<Arc<AppState>>,
    Path(location_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Location,
        "SELECT * FROM locations WHERE location_id = $1",
        location_id
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(location) => (StatusCode::OK, Json(location)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn search_locations(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let result;
    let name = params.get("name");
    if name.is_some() {
        let formatted_name = "%".to_owned() + name.unwrap() + "%";
        result = sqlx::query_as!(
            Location,
            "SELECT * FROM locations WHERE name LIKE $1",
            formatted_name
        )
        .fetch_all(&state.db)
        .await;
    } else {
        result = sqlx::query_as!(Location, "SELECT * FROM locations")
            .fetch_all(&state.db)
            .await;
    }

    match result {
        Ok(location) => (StatusCode::OK, Json(location)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn update_location_by_location_id(
    State(state): State<Arc<AppState>>,
    Path(location_id): Path<Uuid>,
    Json(payload): Json<UpdateLocation>,
) -> impl IntoResponse {
    let result = sqlx::query!(
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
    .await;

    match result {
        Ok(location) => (
            StatusCode::OK,
            Json(json!({
               "location_id": location.location_id,
               "user_id":  location.user_id,
                "name": location.name,
                "environment": location.environment,
                "created_at":  location.created_at,
                "updated_at":  location.updated_at,
            })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_location_by_location_id(
    State(state): State<Arc<AppState>>,
    Path(location_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM locations WHERE location_id = $1", location_id)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
