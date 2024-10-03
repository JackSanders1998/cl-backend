use crate::models::{CreatePreference, Preference};
use crate::routes::{get_claims, preferences_repository, AppState};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use http::HeaderMap;
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/preferences",
    responses(
        (status = 201, description = "Create a preference", body = Preference),
        (status = 500, description = "Preference was not created")
    ),
    request_body = CreatePreference
)]
pub async fn create_preference(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePreference>,
) -> impl IntoResponse {
    let result =
        preferences_repository::create_preference(state, payload, get_claims(headers)).await;

    match result {
        Ok(preference) => (
            StatusCode::CREATED,
            Json(Preference {
                preference_id: preference.preference_id,
                user_id: preference.user_id,
                boulder_scale: preference.boulder_scale,
                sport_scale: preference.sport_scale,
                color_scheme: preference.color_scheme,
                theme: preference.theme,
                created_at: preference.created_at,
                updated_at: preference.updated_at,
            }),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/preferences/{preference_id}",
    params(
        ("preference_id", description = "preference id"),
    ),
    responses(
        (status = 200, description = "Get a preference successfully", body = Preference),
        (status = 404, description = "Preference was not found")
    )
)]
pub async fn get_preference_by_preference_id(
    State(state): State<Arc<AppState>>,
    Path(preference_id): Path<Uuid>,
) -> impl IntoResponse {
    let result =
        preferences_repository::get_preference_by_preference_id(state, preference_id).await;

    match result {
        Ok(preference) => (StatusCode::OK, Json(preference)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/preferences",
    responses(
        (status = 200, description = "Get preference(s) successfully", body = [Preference]),
        (status = 404, description = "No preference found")
    )
)]
pub async fn get_preference_by_user_id(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let result =
        preferences_repository::get_preference_by_user_id(state, get_claims(headers)).await;

    match result {
        Ok(preference) => (StatusCode::OK, Json(preference)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/preferences/{preference_id}",
    params(
        ("preference_id", description = "preference id"),
    ),
    responses(
        (status = 204, description = "Delete a preference"),
        (status = 500, description = "Preference was not deleted")
    )
)]
pub async fn delete_preference(
    State(state): State<Arc<AppState>>,
    Path(preference_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = preferences_repository::delete_preference(state, preference_id).await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
