use std::{str::FromStr, sync::Arc};

use irelia_core::entities::job::JobStatus::InProgress;
use irelia_core::entities::worker_job::WorkerJobStatus::{
    AdditionalBadFlag, IncorrectLayout, IncorrectOffchainProof, NoCairoJobId, Successfully,
};
use irelia_core::entities::worker_job::{
    ProofLayout, WorkerJobEntity, WorkerJobId, WorkerJobStatus,
};
use irelia_core::ports::worker::WorkerPort;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::controllers::worker_job::{CairoPieReq, NewWorkerJob};
use crate::errors::AppError;
use crate::services::job::JobService;

const SUCCESSFULLY_CODE: &str = "JOB_RECEIVED_SUCCESSFULLY";
const INTERNAL_SERVER_ERROR_CODE: &str = "500";
const INTERNAL_SERVER_ERROR_MESSAGE: &str = "Internal server error";

pub struct WorkerJobService {
    worker_job: Arc<dyn WorkerPort + Send + Sync>,
}

impl WorkerJobService {
    pub fn new(worker_job: Arc<dyn WorkerPort + Send + Sync>) -> Self {
        Self { worker_job }
    }

    pub async fn add_worker_job(
        &self,
        job_service: Arc<JobService>,
        params: NewWorkerJob,
        req: CairoPieReq,
    ) -> Result<WorkerJobResponse, AppError> {
        let response_code = Self::check_job(params.clone());

        if matches!(
            response_code,
            IncorrectLayout | NoCairoJobId | IncorrectOffchainProof
        ) {
            return Ok(WorkerJobResponse::get_worker_job_response(response_code));
        }

        let _ = self
            .worker_job
            .add(WorkerJobEntity {
                id: WorkerJobId(Uuid::new_v4()),
                customer_id: params.clone().customer_id,
                cairo_job_key: params.clone().cairo_job_key.unwrap(),
                offchain_proof: params.clone().offchain_proof,
                proof_layout: params.clone().proof_layout,
                cairo_pie: req.request.cairo_pie,
            })
            .await?;

        if response_code == AdditionalBadFlag {
            let _ = job_service.add_job(params.clone(), InProgress, true).await;
        }
        let _ = job_service.add_job(params, InProgress, false).await;

        Ok(WorkerJobResponse::get_worker_job_response(response_code))
    }

    pub fn check_job(params: NewWorkerJob) -> WorkerJobStatus {
        // Check incorrect layout
        match ProofLayout::from_str(params.proof_layout.to_lowercase().as_str()) {
            Ok(_) => (),
            _ => {
                return IncorrectLayout;
            }
        }
        // Check additional bad flag
        if params.bla.is_some() && params.bla.unwrap() {
            return AdditionalBadFlag;
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
            WorkerJobStatus::FaultyCairoPie => Self::successfully(),
            IncorrectLayout => Self::internal_server_error(),
            AdditionalBadFlag => Self::successfully(),
            NoCairoJobId => Self::internal_server_error(),
            IncorrectOffchainProof => Self::internal_server_error(),

            Successfully => Self::successfully(),
        }
    }
}
