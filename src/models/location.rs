use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Location {
    pub location_id: Uuid,
    pub author: String,
    pub name: String,
    pub environment: Environment,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Clone, sqlx::Type, Debug)]
#[sqlx(type_name = "environment", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    Outdoor,
    Gym,
}

impl From<String> for Environment {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Outdoor" => Environment::Outdoor,
            "Gym" => Environment::Gym,
            _ => panic!("Invalid environment"),
        }
    }
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            Environment::Gym => "Gym",
            Environment::Outdoor => "Outdoor",
        }
    }
}
