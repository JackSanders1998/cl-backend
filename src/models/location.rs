use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Location {
    pub location_id: Uuid,
    pub user_id: Option<String>,
    pub name: String,
    pub environment: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateLocation {
    pub user_id: Option<String>,
    pub name: String,
    pub environment: String,
}

#[derive(Deserialize)]
pub struct UpdateLocation {
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub environment: Option<String>,
}
