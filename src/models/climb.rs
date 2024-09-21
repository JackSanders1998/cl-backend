use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Climb {
    pub climb_id: Uuid,
    pub sesh_id: Uuid,
    pub climb_type: ClimbType,
    pub style: Style,
    pub scale: Scale,
    pub grade: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateClimb {
    pub sesh_id: Uuid,
    pub climb_type: ClimbType,
    pub style: Style,
    pub scale: Scale,
    pub grade: String,
}

#[derive(Deserialize)]
pub struct UpdateClimb {
    pub sesh_id: Option<Uuid>,
    pub climb_type: Option<ClimbType>,
    pub style: Option<Style>,
    pub scale: Option<Scale>,
    pub grade: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClimbType {
    Boulder(String),
    Sport(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    TopRope(String),
    Lead(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scale {
    Verm(String),
    Font(String),
    Yosemite(String),
    French(String),
}
