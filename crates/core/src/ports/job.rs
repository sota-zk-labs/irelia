use async_trait::async_trait;

use crate::common::core_error::CoreError;
use crate::entities::job::{JobEntity, JobEntityPayload, JobId};

#[async_trait]
pub trait JobPort {
    async fn add(&self, job: JobEntity) -> Result<JobEntity, CoreError>;
    async fn update(&self, job: JobEntityPayload) -> Result<JobEntity, CoreError>;
    async fn delete(&self, job_id: &JobId) -> Result<(), CoreError>;
    async fn get(&self, customer_id: String, cairo_job_key: String)
        -> Result<JobEntity, CoreError>;
}
