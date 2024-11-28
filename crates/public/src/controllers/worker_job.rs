use std::str;

use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;
use crate::services::worker_job::WorkerJobResponse;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Request {
    pub(crate) cairo_pie: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CairoPieReq {
    pub action: String,
    pub(crate) request: Request,
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
pub async fn add_worker_job(
    State(app_state): State<AppState>,
    Query(params): Query<NewWorkerJob>,
    Json(req): Json<CairoPieReq>,
) -> Result<JsonResponse<WorkerJobResponse>, AppError> {
    let res = app_state
        .worker_service
        .add_worker_job(app_state.job_service, params, req)
        .await?;

    Ok(JsonResponse(res))
}
