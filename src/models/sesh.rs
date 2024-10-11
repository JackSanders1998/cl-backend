use crate::models::{Attempt, ClimbData, ClimbType, LocationData, Scale, Style};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct Sesh {
    pub sesh_id: Uuid,
    pub user_id: String,
    pub notes: Option<String>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub location: LocationData,
    pub climbs: Vec<ClimbData>,
}

#[derive(Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct SqlxSeshWithLocationAndClimbs {
    // sesh
    pub sesh_id: Uuid,
    pub user_id: String,
    pub notes: Option<String>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    // location (inner join)
    pub location_id: Uuid,
    pub name: String,
    pub environment: String,
    pub location_created_at: chrono::DateTime<chrono::Utc>,
    pub location_updated_at: chrono::DateTime<chrono::Utc>,
    // climb (left join)
    pub climb_id: Option<Uuid>,
    pub climb_type: Option<ClimbType>,
    pub style: Option<Style>,
    pub scale: Option<Scale>,
    pub grade: Option<String>,
    pub attempt: Option<Attempt>,
    pub pointer: Option<Uuid>,
    pub climb_notes: Option<String>,
    pub climb_created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub climb_updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSesh {
    pub location_id: Uuid,
    pub notes: Option<String>,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct SeshSearchParams {
    pub notes: Option<String>, //  Add more
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSesh {
    pub location_id: Option<Uuid>,
    pub notes: Option<String>,
    pub end_session: Option<bool>,
}
