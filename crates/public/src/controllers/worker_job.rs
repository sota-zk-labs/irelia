use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;
use crate::services::check::check_worker_job;
use crate::utils::worker_job_response::{get_worker_job_response, WorkerJobResponse};
use axum::body::{Body, Bytes};
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::Json;
use irelia_core::entities::job::JobStatus::{InProgress, Invalid};
use irelia_core::entities::job::{JobEntity, JobId, JobStatus};
use irelia_core::entities::worker_job::WorkerJobStatus::{AdditionalBadFlag, FaultyCairoPie};
use irelia_core::entities::worker_job::{WorkerJob, WorkerJobId, WorkerJobStatus};
use irelia_core::ports::job::JobPort;
use openssl::pkey::Params;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;
use tracing::instrument;
use tracing::log::info;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Request {
    cairo_pie: String,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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
    Json(req): Json<CairoPieReq>,
) -> Result<JsonResponse<WorkerJobResponse>, AppError> {
    info!("params: {:?}, req: {:?}", params, req);
    let (response_code, is_valid) = check_worker_job(params.clone(), req.clone().request.cairo_pie);

    if !is_valid {
        if response_code == FaultyCairoPie {
            let _ = initial_job(app_state.clone(), params.clone(), Invalid, false).await;
        }
        return Ok(JsonResponse(get_worker_job_response(response_code)));
    }

    let _ = app_state
        .clone()
        .worker_port
        .add(WorkerJob {
            id: WorkerJobId(Uuid::new_v4()),
            customer_id: params.clone().customer_id,
            cairo_job_key: params.clone().cairo_job_key.unwrap(),
            offchain_proof: params.clone().offchain_proof,
            proof_layout: params.clone().proof_layout,
            cairo_pie: req.request.cairo_pie,
        })
        .await?;

    if response_code == AdditionalBadFlag {
        let _ = initial_job(app_state.clone(), params.clone(), InProgress, true).await;
        return Ok(JsonResponse(get_worker_job_response(response_code)));
    }

    let _ = initial_job(app_state, params, InProgress, false).await;

    Ok(JsonResponse(get_worker_job_response(response_code)))
}

#[instrument(level = "info", skip(app_state))]
pub async fn initial_job(
    app_state: AppState,
    params: NewWorkerJob,
    job_status: JobStatus,
    validation_done_value: bool,
) {
    let job = app_state
        .job_port
        .add(JobEntity {
            id: JobId(Uuid::new_v4()),
            customer_id: params.clone().customer_id,
            cairo_job_key: params.clone().cairo_job_key.unwrap(),
            status: job_status,
            validation_done: validation_done_value,
        })
        .await
        .expect("Can't initial job");
    info!("{:?}", job);
}
