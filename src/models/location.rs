use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Location {
    pub location_id: Uuid,
    pub author: String,
    pub name: String,
    pub environment: Environment,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateLocation {
    pub author: String,
    pub name: String,
    pub environment: Environment,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateLocation {
    pub name: Option<String>,
    pub environment: Option<Environment>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct LocationSearchParams {
    pub name: Option<String>, //  Add more
}

#[derive(Serialize, Deserialize, Clone, sqlx::Type, Debug, PartialEq, Eq)]
#[sqlx(type_name = "environment_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Gym,
    Outdoor,
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            Environment::Gym => "gym",
            Environment::Outdoor => "outdoor",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "gym" => Some(Environment::Gym),
            "outdoor" => Some(Environment::Outdoor),
            _ => None,
        }
    }
}
