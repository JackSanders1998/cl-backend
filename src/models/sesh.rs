use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Sesh {
    pub sesh_id: Uuid,
    pub user_id: Option<String>,
    pub location_id: Uuid,
    pub notes: Option<String>,
    pub start: chrono::NaiveDateTime,
    pub end: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateSesh {
    pub user_id: Option<String>,
    pub location_id: Uuid,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateSesh {
    pub user_id: Option<String>,
    pub location_id: Option<Uuid>,
    pub notes: Option<String>,
    pub start: Option<chrono::NaiveDateTime>,
    pub end: Option<chrono::NaiveDateTime>,
}
