use crate::models::{CreatePreference, Preference};
use crate::routes::AppState;
use axum::{http::StatusCode, response::IntoResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error as PgError;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_preference(
    state: Arc<AppState>,
    payload: CreatePreference,
    user_id: String,
) -> Result<Preference, PgError> {
    sqlx::query_as(
        r#"
            INSERT INTO preferences (
                user_id,
                boulder_scale,
                sport_scale,
                color_scheme,
                theme
            ) VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(payload.boulder_scale)
    .bind(payload.sport_scale)
    .bind(payload.color_scheme)
    .bind(payload.theme)
    .fetch_one(&state.db)
    .await
}

pub async fn get_preference_by_preference_id(
    state: Arc<AppState>,
    preference_id: Uuid,
) -> Result<Preference, PgError> {
    sqlx::query_as!(
        Preference,
        "SELECT * FROM preferences WHERE preference_id = $1",
        preference_id
    )
    .fetch_one(&state.db)
    .await
}

pub async fn get_preference_by_user_id(
    state: Arc<AppState>,
    user_id: String,
) -> Result<Preference, PgError> {
    sqlx::query_as!(
        Preference,
        "SELECT * FROM preferences WHERE user_id = $1",
        user_id
    )
    .fetch_one(&state.db)
    .await
}

pub async fn delete_preference(
    state: Arc<AppState>,
    preference_id: Uuid,
) -> Result<PgQueryResult, PgError> {
    sqlx::query!(
        "DELETE FROM preferences WHERE preference_id = $1",
        preference_id
    )
    .execute(&state.db)
    .await
}
