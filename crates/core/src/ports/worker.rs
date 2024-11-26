use async_trait::async_trait;

use crate::common::core_error::CoreError;
use crate::entities::worker_job::WorkerJob;

#[async_trait]
pub trait WorkerPort {
    async fn add(&self, job: WorkerJob) -> Result<WorkerJob, CoreError>;
}
