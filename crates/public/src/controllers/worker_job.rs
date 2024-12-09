use std::str;

use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;
use crate::services::worker_job::WorkerJobResponse;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WorkerJob {
    pub customer_id: String,
    pub cairo_job_key: Option<String>,
    pub offchain_proof: bool,
    pub proof_layout: String,
}

#[instrument(level = "info", skip(app_state))]
pub async fn add_worker_job(
    State(app_state): State<AppState>,
    Query(params): Query<WorkerJob>,
    cairo_pie_req: String,
) -> Result<JsonResponse<WorkerJobResponse>, AppError> {
    let res = app_state
        .worker_service
        .add_worker_job(params, cairo_pie_req)
        .await?;

    Ok(JsonResponse(res))
}
