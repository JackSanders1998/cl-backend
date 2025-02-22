use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Route {
    pub route_id: Uuid,
    pub location_id: Uuid,
    pub grade: String,
    pub scale: Scale,
    pub disciplines: Vec<Discipline>,
    pub author: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRoute {
    pub route_id: Uuid,
    pub location_id: Uuid,
    pub grade: String,
    pub scale: Scale,
    pub disciplines: Vec<Discipline>,
    pub author: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRoute {
    pub grade: Option<String>,
    pub scale: Option<Scale>,
    pub disciplines: Option<Vec<Discipline>>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, sqlx::Type, Debug)]
#[sqlx(type_name = "style", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Discipline {
    Boulder,
    Sport,
    TopRope,
}

#[derive(Serialize, Deserialize, Clone, sqlx::Type, Debug)]
#[sqlx(type_name = "scale", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Scale {
    Verm,
    Font,
    Yosemite,
    French,
}