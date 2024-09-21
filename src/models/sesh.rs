use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct Sesh {
    pub sesh_id: Uuid,
    pub user_id: Option<String>,
    pub location_id: Uuid,
    pub notes: Option<String>,
    #[ts(type = "string")]
    pub start: chrono::NaiveDateTime,
    #[ts(type = "string")]
    pub end: Option<chrono::NaiveDateTime>,
    #[ts(type = "string")]
    pub created_at: chrono::NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CreateSesh {
    pub user_id: Option<String>,
    pub location_id: Uuid,
    pub notes: Option<String>,
    #[ts(type = "string")]
    pub start: chrono::NaiveDateTime,
    #[ts(type = "string")]
    pub end: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct UpdateSesh {
    pub user_id: Option<String>,
    pub location_id: Option<Uuid>,
    pub notes: Option<String>,
    #[ts(type = "string")]
    pub start: Option<chrono::NaiveDateTime>,
    #[ts(type = "string")]
    pub end: Option<chrono::NaiveDateTime>,
}
