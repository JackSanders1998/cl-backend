pub use crate::models::{Attempt, ClimbType, Scale, Style};
use crate::models::{Climb, CreateClimb};
use crate::routes::{climbs_repository, AppState};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/climbs",
    responses(
        (status = 201, description = "Create a climb", body = [Climb], content_type = "application/json"),
        (status = 500, description = "Climb was not created", content_type = "application/json")
    ),
    request_body = CreateClimb
)]
pub async fn create_climb(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateClimb>,
) -> impl IntoResponse {
    let result = climbs_repository::create_climb(state, payload).await;

    match result {
        Ok(climb) => (
            StatusCode::CREATED,
            Json(Climb {
                climb_id: climb.climb_id,
                sesh_id: climb.sesh_id,
                climb_type: climb.climb_type,
                style: climb.style,
                scale: climb.scale,
                grade: climb.grade,
                attempt: climb.attempt,
                pointer: climb.pointer,
                notes: climb.notes,
                created_at: climb.created_at,
                updated_at: climb.updated_at,
            }),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/climbs/{climb_id}",
    params(
        ("climb_id", description = "climb id"),
    ),
    responses(
        (status = 200, description = "Get a climb successfully", content_type = "application/json"),
        (status = 404, description = "Climb was not found", content_type = "application/json")
    )
)]
pub async fn get_climb_by_climb_id(
    State(state): State<Arc<AppState>>,
    Path(climb_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = climbs_repository::get_climb_by_climb_id(state, climb_id).await;

    match result {
        Ok(climb) => (StatusCode::OK, Json(climb)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/climbs",
    responses(
        (status = 200, description = "List all climbs successfully", content_type = "application/json"),
        (status = 404, description = "Climb was not found", content_type = "application/json")
    )
)]
pub async fn search_climbs(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let result = climbs_repository::get_all_climbs(state).await;

    match result {
        Ok(climbs) => (StatusCode::OK, Json(climbs)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/climbs/{climb_id}",
    params(
        ("climb_id", description = "climb id"),
    ),
    responses(
        (status = 204, description = "Delete a climb", content_type = "application/json"),
        (status = 500, description = "Climb was not deleted", content_type = "application/json")
    )
)]
pub async fn delete_climb(
    State(state): State<Arc<AppState>>,
    Path(climb_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = climbs_repository::delete_climb(state, climb_id).await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
