use std::sync::Arc;

use irelia_core::entities::job::{JobEntity, JobId, JobStatus};
use irelia_core::ports::job::JobPort;
use serde::{Deserialize, Serialize};
use tracing::log::info;
use uuid::Uuid;

use crate::controllers::job::GetStatusParams;
use crate::controllers::worker_job::NewWorkerJob;
use crate::errors::AppError;

pub struct JobService {
    job: Arc<dyn JobPort + Send + Sync>,
}

impl JobService {
    pub fn new(job: Arc<dyn JobPort + Send + Sync>) -> Self {
        Self { job }
    }

    pub async fn add_job(
        &self,
        params: NewWorkerJob,
        job_status: JobStatus,
        validation_done_value: bool,
    ) -> Result<(), AppError> {
        let job = self
            .job
            .add(JobEntity {
                id: JobId(Uuid::new_v4()),
                customer_id: params.clone().customer_id,
                cairo_job_key: params.clone().cairo_job_key.unwrap(),
                status: job_status,
                invalid_reason: "".to_string(),
                error_log: "".to_string(),
                validation_done: validation_done_value,
            })
            .await
            .expect("Can't initial job");
        info!("{:?}", job);
        Ok(())
    }

    pub async fn get_job_status(&self, params: GetStatusParams) -> Result<JobResponse, AppError> {
        let job = self
            .job
            .get_job(params.customer_id, params.cairo_job_key)
            .await?;
        Ok(JobResponse::get_job_response(job))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JobResponse {
    pub status: String,
    pub invalid_reason: String,
    pub error_log: String,
    pub validation_done: bool,
}

impl JobResponse {
    pub fn get_job_response(job: JobEntity) -> Self {
        JobResponse {
            status: job.status.to_string(),
            invalid_reason: job.invalid_reason,
            error_log: job.error_log,
            validation_done: job.validation_done,
        }
    }
}
