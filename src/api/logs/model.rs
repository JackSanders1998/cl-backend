use serde::Serialize;
use sqlx::FromRow;
use uuid::{uuid, Uuid};

#[derive(Serialize, FromRow)]
pub struct Log {
    pub id: Uuid,
    pub grade: String,
    pub metadata: String,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            id: uuid!("b7f9ddc7-c80d-4bf6-8573-f06e94addfb3"),
            grade: String::from("stub grade"),
            metadata: String::from("stub metadata"),
        }
    }
}

pub trait LogsRepository {
    async fn create_log(&self, grade: String, metadata: String) -> anyhow::Result<Log>;

    async fn get_logs(&self, log_id: Uuid) -> anyhow::Result<Vec<Log>>;
}
