use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Climb {
    pub climb_id: Uuid,
    pub sesh_id: Uuid,
    pub type_: String,
    pub style: String,
    pub scale: String,
    pub grade: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateClimb {
    pub sesh_id: Uuid,
    pub type_: String,
    pub style: String,
    pub scale: String,
    pub grade: String,
}

#[derive(Deserialize)]
pub struct UpdateClimb {
    pub sesh_id: Option<Uuid>,
    pub type_: Option<String>,
    pub style: Option<String>,
    pub scale: Option<String>,
    pub grade: Option<String>,
}