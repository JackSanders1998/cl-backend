use crate::models::{CreatePreference, Preference};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_preference(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePreference>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO preferences (user_id, boulder_scale, sport_scale, color_scheme, theme) VALUES ($1, $2, $3, $4, $5) RETURNING preference_id",
        payload.user_id,
        payload.boulder_scale,
        payload.sport_scale,
        payload.color_scheme,
        payload.theme,
    )
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => (
            StatusCode::CREATED,
            Json(json!({ "preference_id": record.preference_id })),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_preference(
    State(pool): State<PgPool>,
    Path(preference_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Preference,
        "SELECT * FROM preferences WHERE preference_id = $1",
        preference_id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(preference) => (StatusCode::OK, Json(preference)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_preference(
    State(pool): State<PgPool>,
    Path(preference_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM preferences WHERE preference_id = $1", preference_id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
