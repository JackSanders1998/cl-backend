use std::sync::Arc;
use tracing::info;
use uuid::Uuid;
use crate::api::logs::model::Log;


#[derive(Clone)]
pub struct LogsService {
    repository: DynLogsRepository,
}

impl LogsService {
    pub fn new(repository: DynLogsRepository) -> Self {
        Self { repository }
    }
}

impl LogsServiceTrait for LogsService {
    async fn create_log(
        &self,
        grade: String,
        metadata: String,
    ) -> AppResult<Log> {
        let grade = request.name.unwrap();
        let metadata = request.cat_type;

        let created_log = self
            .repository
            .create_log(user_id, name, cat_type)
            .await?;

        info!("user created log successfully");

        Ok(created_log.into_dto())
    }

    async fn get_log_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<CategoryResponseDto> {
        info!("searching for existing log {:?}", id);
        let log = self.repository.get_log_by_id(id).await?;

        if let Some(existing_log) = log {
            // verify the user IDs match on the request and the log
            if existing_log.user_id != user_id {
                return Err(Error::Forbidden);
            }

            return Ok(existing_log.into_dto());
        }

        Err(Error::NotFound(String::from("log was not found")))
    }

    async fn get_categories(&self, user_id: Uuid) -> AppResult<Vec<CategoryResponseDto>> {
        let categories = self.repository.get_categories(user_id).await?;

        self.map_to_categories(categories).await
    }

    async fn updated_log(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: CategoryUpdateDto,
    ) -> AppResult<CategoryResponseDto> {
        let log_to_update = self.repository.get_log_by_id(id).await?;

        if let Some(existing_log) = log_to_update {
            // verify the user IDs match on the request and the log
            if existing_log.user_id != user_id {
                return Err(Error::Forbidden);
            }

            let updated_name = request.name.unwrap_or(existing_log.name);
            let update_cat_type = request.cat_type.unwrap_or(existing_log.cat_type);

            let updated_log = self
                .repository
                .update_log(id, updated_name, update_cat_type)
                .await?;

            return Ok(updated_log.into_dto());
        }

        Err(Error::NotFound(String::from("log was not found")))
    }

    async fn delete_log(&self, user_id: Uuid, id: Uuid) -> AppResult<()> {
        let log = self.repository.get_log_by_id(id).await?;

        if let Some(existing_log) = log {
            // verify the user IDs match on the request and the log
            if existing_log.user_id != user_id {
                return Err(Error::Forbidden);
            }

            self.repository
                .delete_log(existing_log.id)
                .await?;

            return Ok(());
        }

        Err(Error::NotFound(String::from("log was not found")))
    }
}

impl LogsService {
    async fn map_to_categories(
        &self,
        logs: Vec<Category>,
    ) -> AppResult<Vec<CategoryResponseDto>> {
        info!("found {} logs", logs.len());

        let mut mapped_categories: Vec<CategoryResponseDto> = Vec::new();

        if !logs.is_empty() {
            for log in logs {
                mapped_categories.push(log.into_dto());
            }
        }

        Ok(mapped_categories)
    }
}