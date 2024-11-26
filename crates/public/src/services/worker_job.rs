use std::{str::FromStr, sync::Arc};

use irelia_core::entities::job::CairoJobStatus::IN_PROGRESS;
use irelia_core::entities::worker_job::WorkerJobStatus::{
    IncorrectLayout, IncorrectOffchainProof, NoCairoJobId, Successfully,
};
use irelia_core::entities::worker_job::{WorkerJobEntity, WorkerJobId, WorkerJobStatus};
use irelia_core::ports::worker::WorkerPort;
use serde::{Deserialize, Serialize};
use stone_cli::args::LayoutName;
use uuid::Uuid;

use crate::controllers::worker_job::WorkerJob;
use crate::errors::AppError;
use crate::errors::AppError::Unknown;
use crate::services::job::JobService;
use crate::utils::save_cairo_pie;

const SUCCESSFULLY_CODE: &str = "JOB_RECEIVED_SUCCESSFULLY";
const INTERNAL_SERVER_ERROR_CODE: &str = "500";
const INTERNAL_SERVER_ERROR_MESSAGE: &str = "Internal server error";

pub struct WorkerJobService {
    worker_job: Arc<dyn WorkerPort + Send + Sync>,
    job_service: Arc<JobService>,
}

impl WorkerJobService {
    pub fn new(
        worker_job: Arc<dyn WorkerPort + Send + Sync>,
        job_service: Arc<JobService>,
    ) -> Self {
        Self {
            worker_job,
            job_service,
        }
    }

    pub async fn add_worker_job(
        &self,
        job_service: Arc<JobService>,
        params: WorkerJob,
        cairo_pie_req: String,
    ) -> Result<WorkerJobResponse, AppError> {
        let response_code = Self::check_job(&params);

        if matches!(
            response_code,
            IncorrectLayout | NoCairoJobId | IncorrectOffchainProof
        ) {
            return Ok(WorkerJobResponse::get_worker_job_response(response_code));
        }

        let cairo_pie = save_cairo_pie(&cairo_pie_req, params.cairo_job_key.as_ref().unwrap())
            .map_err(|e| Unknown(e))?
            .to_string_lossy()
            .to_string();
        let _ = self
            .worker_job
            .add(WorkerJobEntity {
                id: WorkerJobId(Uuid::new_v4()),
                customer_id: params.customer_id.clone(),
                cairo_job_key: params.cairo_job_key.clone().unwrap(),
                offchain_proof: params.offchain_proof.clone(),
                proof_layout: params.proof_layout.clone(),
                cairo_pie,
            })
            .await?;
        let _ = self.job_service.add_job(params, IN_PROGRESS, false).await;
        Ok(WorkerJobResponse::get_worker_job_response(response_code))
    }

    pub fn check_job(params: &WorkerJob) -> WorkerJobStatus {
        // Check incorrect layout
        match LayoutName::from_str(params.proof_layout.to_lowercase().as_str()) {
            Ok(_) => (),
            _ => {
                return IncorrectLayout;
            }
        }
        // Check no cairo job id
        if params.cairo_job_key.is_none() {
            return NoCairoJobId;
        };

        // check incorrect off chain proof
        if !params.offchain_proof {
            return IncorrectOffchainProof;
        };

        // Successfully
        Successfully
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WorkerJobResponse {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl WorkerJobResponse {
    pub fn successfully() -> Self {
        WorkerJobResponse {
            code: SUCCESSFULLY_CODE.to_string(),
            message: None,
        }
    }

    pub fn internal_server_error() -> Self {
        WorkerJobResponse {
            code: INTERNAL_SERVER_ERROR_CODE.to_string(),
            message: Some(INTERNAL_SERVER_ERROR_MESSAGE.to_string()),
        }
    }

    pub fn get_worker_job_response(code: WorkerJobStatus) -> Self {
        match code {
            IncorrectLayout => Self::internal_server_error(),
            NoCairoJobId => Self::internal_server_error(),
            IncorrectOffchainProof => Self::internal_server_error(),

            Successfully => Self::successfully(),
        }
    }
}
