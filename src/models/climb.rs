use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Climb {
    pub climb_id: Uuid,
    pub sesh_id: Uuid,
    pub climb_type: ClimbType,
    pub style: Option<Style>,
    pub scale: Scale,
    pub grade: String,
    pub attempt: Attempt,
    pub pointer: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateClimb {
    pub sesh_id: Uuid,
    pub climb_type: ClimbType,
    pub style: Option<Style>,
    pub scale: Scale,
    pub grade: String,
    pub attempt: Attempt,
    pub pointer: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateClimb {
    pub sesh_id: Option<Uuid>,
    pub climb_type: Option<ClimbType>,
    pub style: Option<Style>,
    pub scale: Option<Scale>,
    pub grade: Option<String>,
    pub attempt: Option<Attempt>,
    pub pointer: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "climb_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ClimbType {
    Boulder,
    Sport,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "style", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Style {
    TopRope,
    Lead,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "scale", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Scale {
    Verm,
    Font,
    Yosemite,
    French,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "attempt", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Attempt {
    Onsight,
    Flash,
    Redpoint,
    Fall,
}
