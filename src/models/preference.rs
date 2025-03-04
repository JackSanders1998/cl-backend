use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Preference {
    pub preference_id: Uuid,
    pub user_id: String,
    pub boulder_scale: String,
    pub sport_scale: String,
    pub color_scheme: String,
    pub theme: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct CreatePreference {
    pub boulder_scale: String,
    pub sport_scale: String,
    pub color_scheme: String,
    pub theme: String,
}

#[derive(Deserialize)]
pub struct UpdatePreference {
    pub boulder_scale: Option<String>,
    pub sport_scale: Option<String>,
    pub color_scheme: Option<String>,
    pub theme: Option<String>,
}
