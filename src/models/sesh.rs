use crate::models::{Attempt, ClimbData, ClimbType, CreateLocation, Scale, Style};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Sesh {
    pub sesh_id: Uuid,
    pub user_id: String,
    pub location_id: Uuid,
    pub notes: Option<String>,
    pub start: chrono::NaiveDateTime,
    pub end: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct SqlxSeshWithLocationAndClimbs {
    // sesh
    pub sesh_id: Uuid,
    pub user_id: String,
    pub location_id: Uuid,
    pub notes: Option<String>,
    pub start: chrono::NaiveDateTime,
    pub end: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    // location
    pub name: String,
    pub environment: String,
    // climb
    pub climb_type: ClimbType,
    pub style: Option<Style>,
    pub scale: Scale,
    pub grade: String,
    pub attempt: Attempt,
    pub pointer: Option<Uuid>,
    pub climb_notes: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SeshWithLocationAndClimbs {
    pub sesh_id: Uuid,
    pub user_id: String,
    pub location_id: Uuid,
    pub notes: Option<String>,
    pub start: chrono::NaiveDateTime,
    pub end: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub location: CreateLocation,
    pub climbs: Vec<ClimbData>,
}

#[derive(Deserialize)]
pub struct CreateSesh {
    pub location_id: Uuid,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateSesh {
    pub location_id: Option<Uuid>,
    pub notes: Option<String>,
    pub end_session: Option<bool>,
}
