use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, ToSchema)]
pub struct Workout {
    pub workout_id: Uuid,
    pub sesh_id: Uuid,
    pub user_id: String,
    pub log: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CreateWorkout {
    pub sesh_id: Uuid,
    pub log: serde_json::Value,
}
