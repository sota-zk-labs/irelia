use std::sync::Arc;

use irelia_core::entities::job::{CairoJobStatus, JobEntity, JobId};
use irelia_core::ports::job::JobPort;
use serde::{Deserialize, Serialize};
use tracing::log::debug;
use uuid::Uuid;

use crate::controllers::job::GetStatusParams;
use crate::controllers::worker_job::WorkerJob;
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
        params: WorkerJob,
        job_status: CairoJobStatus,
        validation_done_value: bool,
    ) -> Result<(), AppError> {
        let job = self
            .job
            .add(JobEntity {
                id: JobId(Uuid::new_v4()),
                customer_id: params.customer_id,
                cairo_job_key: params.cairo_job_key.unwrap(),
                status: job_status,
                invalid_reason: Default::default(),
                error_log: Default::default(),
                validation_done: validation_done_value,
            })
            .await?;
        debug!("{:?}", job);
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
