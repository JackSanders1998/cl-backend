use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use ts_rs::TS;

#[derive(Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct Location {
    pub location_id: Uuid,
    pub user_id: Option<String>,
    pub name: String,
    pub environment: String,
    #[ts(type = "string")]
    pub created_at: chrono::NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CreateLocation {
    pub user_id: Option<String>,
    pub name: String,
    pub environment: String,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct UpdateLocation {
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub environment: Option<String>,
}
