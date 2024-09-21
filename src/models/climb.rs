use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use ts_rs::TS;

#[derive(Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct Climb {
    pub climb_id: Uuid,
    pub sesh_id: Uuid,
    pub climb_type: String,
    pub style: String,
    pub scale: String,
    pub grade: String,
    #[ts(type = "string")]
    pub created_at: chrono::NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CreateClimb {
    pub sesh_id: Uuid,
    pub climb_type: String,
    pub style: String,
    pub scale: String,
    pub grade: String,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct UpdateClimb {
    pub sesh_id: Option<Uuid>,
    pub climb_type: Option<String>,
    pub style: Option<String>,
    pub scale: Option<String>,
    pub grade: Option<String>,
}
