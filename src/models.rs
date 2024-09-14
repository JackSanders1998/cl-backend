use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Location {
    pub location_id: Uuid,
    pub user_id: Option<String>,
    pub name: String,
    pub environment: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateLocation {
    pub user_id: Option<String>,
    pub name: String,
    pub environment: String,
}

#[derive(Deserialize)]
pub struct UpdateLocation {
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub environment: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Sesh {
    pub sesh_id: Uuid,
    pub user_id: Option<String>,
    pub location_id: Uuid,
    pub notes: Option<String>,
    pub start: chrono::NaiveDateTime,
    pub end: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateSesh {
    pub user_id: Option<String>,
    pub location_id: Uuid,
    pub notes: Option<String>,
    pub start: chrono::NaiveDateTime,
    pub end: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct UpdateSesh {
    pub user_id: Option<String>,
    pub location_id: Option<Uuid>,
    pub notes: Option<String>,
    pub start: Option<chrono::NaiveDateTime>,
    pub end: Option<chrono::NaiveDateTime>,
}

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

#[derive(Serialize, Deserialize, FromRow)]
pub struct Preference {
    pub preference_id: Uuid,
    pub user_id: String,
    pub boulder_scale: String,
    pub sport_scale: String,
    pub color_scheme: String,
    pub theme: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreatePreference {
    pub user_id: String,
    pub boulder_scale: String,
    pub sport_scale: String,
    pub color_scheme: String,
    pub theme: String,
}

#[derive(Deserialize)]
pub struct UpdatePreference {
    pub boulder_scale: Option<String>,
    pub sport_scale: Option<String>,
    pub color_scheme: Option<String>,
    pub theme: Option<String>,
}
