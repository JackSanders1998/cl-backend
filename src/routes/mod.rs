mod climbs;
mod health_check;
mod locations;
mod preferences;
mod seshes;

use clerk_rs::clerk::Clerk;
use clerk_rs::validators::authorizer::ClerkAuthorizer;
pub use climbs::*;
pub use health_check::health_check;
pub use locations::*;
pub use preferences::*;
pub use seshes::*;
use sqlx::{Pool, Postgres};

pub struct AppState {
    pub db: Pool<Postgres>,
    pub auth: ClerkAuthorizer,
    pub clerk: Clerk,
}
