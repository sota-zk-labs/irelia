use std::collections::HashMap;
use std::str;
use axum::body::{Body, Bytes};
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::Json;
use irelia_core::entities::worker_job::{WorkerJob, WorkerJobId};
use irelia_core::entities::job::{JobEntity, JobId, JobStatus};
use irelia_core::ports::job::JobPort;
use openssl::pkey::Params;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use tracing::log::info;
use uuid::Uuid;
use irelia_core::entities::job::JobStatus::InProgress;

use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;
use crate::services::check::check_worker_job;
use crate::utils::worker_job_response::{get_worker_job_response, WorkerJobResponse};

#[derive(Debug, Deserialize)]
pub struct Request {
    cairo_pie: String,
}
#[derive(Debug, Deserialize)]
pub struct CairoPieReq {
    action: String,
    request: Request,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NewWorkerJob {
    pub customer_id: String,
    pub cairo_job_key: Option<String>,
    pub offchain_proof: bool,
    pub proof_layout: String,
    pub bla: Option<bool>,
}

#[instrument(level = "info", skip(app_state))]
pub async fn add_job(
    State(app_state): State<AppState>,
    Query(params): Query<NewWorkerJob>,
    Json(req): Json<CairoPieReq>
) -> Result<JsonResponse<WorkerJobResponse>, AppError> {
    info!("{:?}", params);
    let (response_code, is_valid) = check_worker_job(params.clone());

    if !is_valid  {
        return  Ok(JsonResponse(get_worker_job_response(response_code)))
    }

    let worker_job = app_state
        .worker_port
        .add(WorkerJob {
            id: WorkerJobId(Uuid::new_v4()),
            customer_id: params.customer_id,
            cairo_job_key: params.cairo_job_key.unwrap(),
            offchain_proof: params.offchain_proof,
            proof_layout: params.proof_layout,
            cairo_pie: req.request.cairo_pie,
        })
        .await?;

    // TODO: Process the data
    let _ = app_state
        .job_port
        .add(JobEntity {
            id: JobId(Uuid::new_v4()),
            customer_id: worker_job.customer_id.clone(),
            cairo_job_key: worker_job.cairo_job_key.clone(),
            status: InProgress,
            validation_done: false,
        })
        .await?;

    Ok(JsonResponse(get_worker_job_response(response_code)))
}