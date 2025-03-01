use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Clone, sqlx::Type, Debug, PartialEq, Eq)]
#[sqlx(type_name = "discipline_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Discipline {
    Boulder,
    Sport,
    TopRope,
}

impl Discipline {
    pub fn as_str(&self) -> &str {
        match self {
            Discipline::Boulder => "boulder",
            Discipline::Sport => "sport",
            Discipline::TopRope => "top_rope",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "boulder" => Some(Discipline::Boulder),
            "sport" => Some(Discipline::Sport),
            "top_rope" => Some(Discipline::TopRope),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, sqlx::Type, Debug, PartialEq, Eq)]
#[sqlx(type_name = "scale", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Scale {
    Verm,
    Font,
    Yosemite,
    French,
}

impl Scale {
    pub fn as_str(&self) -> &str {
        match self {
            Scale::Verm => "verm",
            Scale::Font => "font",
            Scale::Yosemite => "yosemite",
            Scale::French => "french",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "verm" => Some(Scale::Verm),
            "font" => Some(Scale::Font),
            "yosemite" => Some(Scale::Yosemite),
            "french" => Some(Scale::French),
            _ => None,
        }
    }
}
