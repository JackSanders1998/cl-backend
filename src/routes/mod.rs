pub mod health_check;
pub mod locations;
pub mod preferences;
pub mod routes;
pub mod seshes;
pub mod ticks;

pub use locations::*;
pub use preferences::*;
pub use routes::*;
pub use seshes::*;
pub use ticks::*;

use base64::engine::general_purpose;
use base64::{alphabet, engine, Engine};
pub use health_check::health_check;
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
}

fn get_claims(header: HeaderMap) -> String {
    #[derive(Serialize, Deserialize)]
    struct Claims {
        sub: String,
    }

    let payload = header
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .split(".")
        .collect::<Vec<_>>()[1];

    let bytes = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
        .decode(payload)
        .unwrap();

    let claims: Claims = serde_json::from_str(String::from_utf8(bytes).unwrap().as_str()).unwrap();

    return claims.sub;
}
