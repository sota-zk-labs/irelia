use async_trait::async_trait;

use crate::common::core_error::CoreError;
use crate::entities::worker_job::{WorkerJob, WorkerJobId};

#[async_trait]
pub trait WorkedJobPort {
    async fn add(&self, job: WorkerJob) -> Result<WorkerJob, CoreError>;
    async fn update(&self, job: WorkerJob) -> Result<WorkerJob, CoreError>;
    async fn delete(&self, job_id: &WorkerJobId) -> Result<(), CoreError>;
    async fn get(&self, job_id: &WorkerJobId) -> Result<WorkerJob, CoreError>;
}
