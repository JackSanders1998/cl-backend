use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Location {
    pub location_id: Uuid,
    pub user_id: String,
    pub name: String,
    pub environment: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LocationData {
    pub location_id: Uuid,
    pub name: String,
    pub environment: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateLocation {
    pub name: String,
    pub environment: String,
}

#[derive(Deserialize)]
pub struct UpdateLocation {
    pub name: Option<String>,
    pub environment: Option<String>,
}

#[derive(Deserialize)]
pub struct LocationSearchParams {
    pub name: Option<String>, //  Add more
}
