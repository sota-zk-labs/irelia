use async_trait::async_trait;

use crate::common::core_error::CoreError;
use crate::entities::job::JobEntity;

#[async_trait]
pub trait WorkerPort {
    async fn add(&self, job: JobEntity) -> Result<JobEntity, CoreError>;
}
