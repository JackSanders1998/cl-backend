use sqlx::{query_as};
use uuid::Uuid;

use crate::database::Database;

use super::model::{Log, LogsRepository};

impl LogsRepository for Database {
    async fn create_log(&self, grade: String, metadata: String) -> anyhow::Result<Log> {
        query_as!(
            Log,
            "INSERT INTO logs (grade, metadata) VALUES ($1, $2) RETURNING id, grade, metadata",
            grade,
            metadata,
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occurred while creating the log")
    }

    async fn get_logs(&self, id: Uuid) -> anyhow::Result<Vec<Log>> {
        query_as!(Log, "SELECT * FROM logs",)
            .fetch_all(&self.pool)
            .await
            .context("log was not found")
    }
}
