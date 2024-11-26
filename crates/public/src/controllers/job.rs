use std::collections::HashMap;
use std::str;

use axum::body::Bytes;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use irelia_adapter::repositories::postgres::status_db::StatusJobDBRepository;
use irelia_core::entities::job::{JobEntity, JobId, JobResponse};
use irelia_core::entities::status::{StatusEntity, StatusId};
use irelia_core::ports::status::StatusPort;
use openssl::pkey::Params;
use tracing::instrument;
use tracing::log::info;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;

#[instrument(level = "info", skip(app_state))]
pub async fn add_job(
    State(app_state): State<AppState>,
    body: Bytes,
) -> Result<JsonResponse<Vec<JobResponse>>, AppError> {
    // TODO: Process the data
    let data = str::from_utf8(&body).unwrap();
    info!("{}", data);

    let job_entity = app_state
        .worker_port
        .add(JobEntity {
            id: JobId(Uuid::new_v4()),
            customer_id: "1".to_string(),
            cairo_job_key: "1".to_string(),
            offchain_proof: false,
            proof_layout: "1".to_string(),
            cairo_pie: "1".to_string(),
        })
        .await?;

    let job_status_repo = StatusJobDBRepository::new(app_state.db.clone());
    let job_status = job_status_repo
        .add(StatusEntity {
            id: StatusId(Uuid::new_v4()),
            customer_id: job_entity.customer_id.clone(),
            cairo_job_key: job_entity.cairo_job_key.clone(),
            status: "Pending".to_string(),
            validation_done: false,
        })
        .await?;

    Ok(JsonResponse(vec![JobResponse {
        code: Some("JOB_RECEIVED_SUCCESSFULLY".to_string()),
        message: Some(job_entity.id.0.to_string()),
    }]))
}
