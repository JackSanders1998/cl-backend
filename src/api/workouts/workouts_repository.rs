use crate::api::AppState;
use crate::models::{CreateWorkout, Workout};
use sqlx::postgres::PgQueryResult;
use sqlx::Error as PgError;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_workout(
    state: Arc<AppState>,
    payload: CreateWorkout,
    user_id: String,
) -> Result<Workout, PgError> {
    sqlx::query_as(
        r#"
            INSERT INTO workouts (
                sesh_id,
                user_id,
                log
            ) VALUES ($1, $2, $3)
            RETURNING *;
        "#,
    )
    .bind(payload.sesh_id)
    .bind(user_id)
    .bind(payload.log)
    .fetch_one(&state.db)
    .await
}

pub async fn get_workout_by_workout_id(
    state: Arc<AppState>,
    workout_id: Uuid,
) -> Result<Workout, PgError> {
    sqlx::query_as(
        r#"
            SELECT *
            FROM workouts
            WHERE workout_id = $1;
        "#,
    )
    .bind(workout_id)
    .fetch_one(&state.db)
    .await
}

pub async fn get_all_workouts(state: Arc<AppState>) -> Result<Vec<Workout>, PgError> {
    sqlx::query_as!(Workout, "SELECT * FROM workouts")
        .fetch_all(&state.db)
        .await
}

pub async fn delete_workout(
    state: Arc<AppState>,
    workout_id: Uuid,
) -> Result<PgQueryResult, PgError> {
    sqlx::query!("DELETE FROM workouts WHERE workout_id = $1", workout_id)
        .execute(&state.db)
        .await
}
