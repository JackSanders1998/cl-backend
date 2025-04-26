use crate::api::{workouts_repository, AppState};
use crate::models::{CreateWorkout, Workout};
use axum::extract::{Path, State};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/workouts",
    request_body = CreateWorkout,
    responses(
        (status = 201, description = "Create a workout", body = Workout),
        (status = 500, description = "Workout was not created")
    )
)]
pub async fn create_workout(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateWorkout>,
) -> impl IntoResponse {
    info!("Creating workout with payload: {:?}", payload);
    let result = workouts_repository::create_workout(state, payload).await;

    match result {
        Ok(workout) => (
            StatusCode::CREATED,
            Json(Workout {
                workout_id: workout.workout_id,
                sesh_id: workout.sesh_id,
                user_id: workout.user_id,
                log: workout.log,
                created_at: workout.created_at,
                updated_at: workout.updated_at,
            }),
        )
            .into_response(),
        Err(error) => {
            error!("Failed to get create a workout. Error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/workouts/{workout_id}",
    params(
        ("workout_id", description = "workout id"),
    ),
    responses(
        (status = 200, description = "Got a workout successfully", body = Workout),
        (status = 404, description = "Workout was not found")
    )
)]
pub async fn get_workout_by_workout_id(
    State(state): State<Arc<AppState>>,
    Path(workout_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = workouts_repository::get_workout_by_workout_id(state, workout_id).await;

    match result {
        Ok(workout) => (StatusCode::OK, Json(workout)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/workouts",
    // params(some param),
    responses(
        (status = 200, description = "Get workout(s) successfully", body = [Workout]),
        (status = 404, description = "No workouts found")
    )
)]
pub async fn search_workouts(
    State(state): State<Arc<AppState>>,
    // Query(params): Query<some param>,
) -> impl IntoResponse {
    let result = workouts_repository::get_all_workouts(state).await;

    match result {
        Ok(workouts) => (StatusCode::OK, Json(workouts)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/workouts/{workout_id}",
    params(
        ("workout_id", description = "workout id"),
    ),
    responses(
        (status = 204, description = "Delete a workout"),
        (status = 500, description = "Workout was not deleted")
    )
)]
pub async fn delete_workout(
    State(state): State<Arc<AppState>>,
    Path(workout_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = workouts_repository::delete_workout(state, workout_id).await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
