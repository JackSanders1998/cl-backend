use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Location {
    pub location_id: Uuid,
    pub user_id: String,
    pub name: String,
    pub environment: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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
