use async_trait::async_trait;

use crate::common::core_error::CoreError;
use crate::entities::job::{Job, JobId};

#[async_trait]
pub trait JobPort {
    async fn add(&self, job: Job) -> Result<Job, CoreError>;
    async fn update(&self, job: Job) -> Result<Job, CoreError>;
    async fn delete(&self, job_id: &JobId) -> Result<(), CoreError>;
    async fn get(&self, job_id: &JobId) -> Result<Job, CoreError>;
}
