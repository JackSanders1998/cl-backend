use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::Discipline;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Tick {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTick {
    pub route_id: Uuid,
    pub sesh_id: Uuid,
    pub discipline: Discipline,
    pub attempt: Attempt,
    pub notes: Option<String>,
    pub lap_group: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickSearchParams {
    pub sesh_id: Uuid,
}

#[derive(Serialize, Deserialize, Clone, sqlx::Type, Debug)]
#[sqlx(type_name = "attempt", rename_all = "snake_case")]
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