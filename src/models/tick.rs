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

#[derive(Serialize, Deserialize)]
pub struct CreateTick {
    pub route_id: Uuid,
    pub sesh_id: Uuid,
    pub discipline: Discipline,
    pub attempt: Attempt,
    pub notes: Option<String>,
    pub lap_group: Option<Uuid>,
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

