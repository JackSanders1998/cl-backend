use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use ts_rs::TS;

#[derive(Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct Preference {
    pub preference_id: Uuid,
    pub user_id: String,
    pub boulder_scale: String,
    pub sport_scale: String,
    pub color_scheme: String,
    pub theme: String,
    #[ts(type = "string")]
    pub created_at: chrono::NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CreatePreference {
    pub user_id: String,
    pub boulder_scale: String,
    pub sport_scale: String,
    pub color_scheme: String,
    pub theme: String,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct UpdatePreference {
    pub boulder_scale: Option<String>,
    pub sport_scale: Option<String>,
    pub color_scheme: Option<String>,
    pub theme: Option<String>,
}
