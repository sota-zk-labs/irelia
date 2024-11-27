use std::collections::HashMap;
use std::str;

use axum::body::Bytes;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::Json;
use irelia_core::entities::worker_job::{WorkerJob, WorkerJobId, WorkerJobResponse, NewWorkerJob};
use irelia_core::entities::job::{JobEntity, JobId, JobStatus};
use irelia_core::ports::job::JobPort;
use openssl::pkey::Params;
use tracing::instrument;
use tracing::log::info;
use uuid::Uuid;
use irelia_core::entities::job::JobStatus::InProgress;
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;

#[instrument(level = "info", skip(app_state))]
pub async fn add_job(
    State(app_state): State<AppState>,
    Json(req): Json<NewWorkerJob>,
) -> Result<JsonResponse<Vec<WorkerJobResponse>>, AppError> {

    info!("{:?}", req);
    let worker_job = app_state
        .worker_port
        .add(WorkerJob {
            id: WorkerJobId(Uuid::new_v4()),
            customer_id: req.customer_id,
            cairo_job_key: req.cairo_job_key,
            offchain_proof: req.offchain_proof,
            proof_layout: req.proof_layout.to_string(),
            cairo_pie: req.cairo_pie,
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

    Ok(JsonResponse(vec![WorkerJobResponse {
        code: Some("JOB_RECEIVED_SUCCESSFULLY".to_string()),
        message: Some(worker_job.id.0.to_string()),
    }]))
}
