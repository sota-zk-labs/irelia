use std::fmt::Debug;

use async_trait::async_trait;

use crate::common::core_error::CoreError;
use crate::entities::job::{JobEntity, JobId};

#[async_trait]
pub trait JobPort: Debug {
    async fn add(&self, job: JobEntity) -> Result<JobEntity, CoreError>;
    async fn update(&self, job: JobEntity) -> Result<JobEntity, CoreError>;
    async fn delete(&self, job_id: &JobId) -> Result<(), CoreError>;
    async fn get_by_customer_id_and_cairo_job_key_value(
        &self,
        customer_id: String,
        cairo_job_key: String,
    ) -> Result<JobEntity, CoreError>;
}
