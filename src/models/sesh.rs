use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{tick::Attempt, Discipline, Environment, Location, Scale};

// Literal representation of the seshes table in the database
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct SeshRow {
    pub sesh_id: Uuid,
    pub user_id: String,
    pub location_id: Uuid,
    pub notes: Option<String>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Sesh {
    pub sesh_id: Uuid,
    pub user_id: String,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    pub location: Location,
    pub ticks: Vec<TickQuery>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct SeshFromRow {
    // sesh
    pub sesh_id: Uuid,
    pub user_id: String,
    pub notes: Option<String>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    // location (inner join)
    pub author: String,
    pub name: String,
    pub environment: Environment,
    pub description: Option<String>,
    pub location_created_at: chrono::DateTime<chrono::Utc>,
    pub location_updated_at: chrono::DateTime<chrono::Utc>,
    // tick (left join)
    pub tick_id: Uuid,
    pub discipline: Discipline,
    pub attempt: Attempt,
    pub tick_notes: Option<String>,
    pub lap_group: Option<Uuid>,
    pub tick_created_at: chrono::DateTime<chrono::Utc>,
    pub tick_updated_at: chrono::DateTime<chrono::Utc>,
    // route (left join)
    pub route_id: Uuid,
    pub location_id: Uuid,
    pub grade: String,
    pub scale: Scale,
    pub disciplines: Vec<Discipline>,
    pub route_author: String,
    pub route_description: Option<String>,
    pub route_created_at: chrono::DateTime<chrono::Utc>,
    pub route_updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct TickQuery {
    pub tick_id: Uuid,
    pub route_id: Uuid,
    pub discipline: Discipline,
    pub attempt: Attempt,
    pub tick_notes: Option<String>,
    pub lap_group: Option<Uuid>,
    pub tick_created_at: chrono::DateTime<chrono::Utc>,
    pub tick_updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CreateSesh {
    pub location_id: Uuid,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct SeshSearchParams {
    pub notes: Option<String>, //  Add more
}

#[derive(Deserialize)]
pub struct UpdateSesh {
    pub location_id: Option<Uuid>,
    pub notes: Option<String>,
    pub end_session: Option<bool>,
}
