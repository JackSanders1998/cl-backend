use crate::models::{Environment, Location, TickWithRoute};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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
pub struct SeshWithLocation {
    pub sesh_id: Uuid,
    pub user_id: String,
    pub notes: Option<String>,
    pub location: Location,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct SeshWithLocationSqlx {
    pub sesh_id: Uuid,
    pub user_id: String,
    pub notes: Option<String>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub location_id: Uuid,
    pub author: String,
    pub name: String,
    pub environment: Environment,
    pub description: Option<String>,
    pub location_created_at: chrono::DateTime<chrono::Utc>,
    pub location_updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct SeshWithLocationAndTicks {
    pub sesh_id: Uuid,
    pub user_id: String,
    pub notes: Option<String>,
    pub location: Location,
    pub ticks: Vec<TickWithRoute>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSesh {
    pub location_id: Uuid,
    pub notes: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateSesh {
    pub location_id: Option<Uuid>,
    pub notes: Option<String>,
    pub end_session: Option<bool>,
}
