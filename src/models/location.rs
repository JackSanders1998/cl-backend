use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct Location {
    pub location_id: Uuid,
    pub user_id: String,
    pub name: String,
    pub environment: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateLocation {
    pub name: String,
    pub environment: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateLocation {
    pub name: Option<String>,
    pub environment: Option<String>,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct LocationSearchParams {
    pub name: Option<String>, //  Add more
}
