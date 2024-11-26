use async_trait::async_trait;

use crate::common::core_error::CoreError;
use crate::entities::status::{StatusEntity, StatusId};

#[async_trait]
pub trait StatusPort {
    async fn add(&self, job: StatusEntity) -> Result<StatusEntity, CoreError>;
    async fn update(&self, job: StatusEntity) -> Result<StatusEntity, CoreError>;
    async fn delete(&self, job_id: &StatusId) -> Result<(), CoreError>;
    async fn get(&self, job_id: &StatusId) -> Result<StatusEntity, CoreError>;
}
