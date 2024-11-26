use std::collections::HashMap;
use std::str;

use axum::body::Bytes;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::Json;
use irelia_adapter::repositories::postgres::job_db::JobDBRepository;
use irelia_core::entities::worker_job::{WorkerJob, WorkerJobId, JobResponse, NewWorkerJob};
use irelia_core::entities::job::{Job, StatusId};
use irelia_core::ports::job::StatusPort;
use openssl::pkey::Params;
use tracing::instrument;
use tracing::log::info;
use uuid::Uuid;
use irelia_adapter::repositories::postgres::schema::jobs::{cairo_job_key, proof_layout};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;

#[instrument(level = "info", skip(app_state))]
pub async fn add_job(
    State(app_state): State<AppState>,
    Json(req): Json<NewWorkerJob>,
) -> Result<JsonResponse<Vec<JobResponse>>, AppError> {
    // TODO: Process the data
    info!("{:?}", req);

    let worked_job = app_state
        .worker_port
        .add(WorkerJob {
            id: WorkerJobId(Uuid::new_v4()),
            customer_id: req.customer_id,
            cairo_job_key: req.cairo_job_key,
            offchain_proof: req.offchain_proof,
            proof_layout: req.proof_layout,
            cairo_pie: req.cairo_pie,
        })
        .await?;

    let job_status_repo = JobDBRepository::new(app_state.db.clone());

    let _ = job_status_repo
        .add(Job {
            id: StatusId(Uuid::new_v4()),
            customer_id: worked_job.customer_id.clone(),
            cairo_job_key: worked_job.cairo_job_key.clone(),
            status: "Pending".to_string(),
            validation_done: false,
        })
        .await?;

    Ok(JsonResponse(vec![JobResponse {
        code: Some("JOB_RECEIVED_SUCCESSFULLY".to_string()),
        message: Some(worked_job.id.0.to_string()),
    }]))
}
