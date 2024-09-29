use crate::models::{CreatePreference, Preference};
use crate::routes::{get_claims, AppState};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use http::HeaderMap;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_preference(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePreference>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO preferences (user_id, boulder_scale, sport_scale, color_scheme, theme) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        get_claims(headers),
        payload.boulder_scale,
        payload.sport_scale,
        payload.color_scheme,
        payload.theme,
    )
        .fetch_one(&state.db)
        .await;

    match result {
        Ok(preference) => (
            StatusCode::CREATED,
            Json(json!({
                "preference_id": preference.preference_id,
                "userId": preference.user_id,
                "boulder_scale": preference.boulder_scale,
                "sport_scale": preference.sport_scale,
                "color_scheme": preference.color_scheme,
                "theme": preference.theme,
                "createdAt": preference.created_at,
                "updatedAt": preference.updated_at,
            })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_preference_by_preference_id(
    State(state): State<Arc<AppState>>,
    Path(preference_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Preference,
        "SELECT * FROM preferences WHERE preference_id = $1",
        preference_id
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(preference) => (StatusCode::OK, Json(preference)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_preference_by_user_id(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Preference,
        "SELECT * FROM preferences WHERE user_id = $1",
        get_claims(headers)
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(preference) => (StatusCode::OK, Json(preference)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_preference(
    State(state): State<Arc<AppState>>,
    Path(preference_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "DELETE FROM preferences WHERE preference_id = $1",
        preference_id
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
