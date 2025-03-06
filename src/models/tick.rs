use crate::models::{Discipline, Environment, RouteWithLocation, Scale};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, ToSchema)]
pub struct TickRow {
    pub tick_id: Uuid,
    pub route_id: Uuid,
    pub sesh_id: Uuid,
    pub discipline: Discipline,
    pub attempt: Attempt,
    pub notes: Option<String>,
    pub lap_group: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone, ToSchema)]
pub struct TickWithRoute {
    pub tick_id: Uuid,
    pub route_id: Uuid,
    pub sesh_id: Uuid,
    pub discipline: Discipline,
    pub attempt: Attempt,
    pub notes: Option<String>,
    pub lap_group: Option<Uuid>,
    pub route: RouteWithLocation,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, ToSchema)]
pub struct TickWithRouteSqlx {
    // tick
    pub tick_id: Uuid,
    pub sesh_id: Uuid,
    pub discipline: Discipline,
    pub attempt: Attempt,
    pub notes: Option<String>,
    pub lap_group: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    // route
    pub route_id: Uuid,
    pub grade: String,
    pub scale: Scale,
    pub disciplines: Vec<Discipline>,
    pub author: String,
    pub description: Option<String>,
    pub route_created_at: chrono::DateTime<chrono::Utc>,
    pub route_updated_at: chrono::DateTime<chrono::Utc>,
    // location
    pub location_id: Uuid,
    pub location_author: String,
    pub name: String,
    pub environment: Environment,
    pub location_description: Option<String>,
    pub location_created_at: chrono::DateTime<chrono::Utc>,
    pub location_updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreateTick {
    pub route_id: Uuid,
    pub sesh_id: Uuid,
    pub discipline: Discipline,
    pub attempt: Attempt,
    pub notes: Option<String>,
    pub lap_group: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, IntoParams)]
pub struct TickSearchParams {
    pub sesh_id: Uuid,
}

#[derive(Serialize, Deserialize, Clone, sqlx::Type, Debug, ToSchema)]
#[sqlx(type_name = "attempt_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Attempt {
    Onsight,
    Flash,
    Redpoint,
    Fall,
}

impl Attempt {
    pub fn as_str(&self) -> &str {
        match self {
            Attempt::Onsight => "onsight",
            Attempt::Flash => "flash",
            Attempt::Redpoint => "redpoint",
            Attempt::Fall => "fall",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "onsight" => Some(Attempt::Onsight),
            "flash" => Some(Attempt::Flash),
            "redpoint" => Some(Attempt::Redpoint),
            "fall" => Some(Attempt::Fall),
            _ => None,
        }
    }
}
