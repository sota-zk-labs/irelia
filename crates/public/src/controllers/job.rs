use axum::extract::{Query, State};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::instrument;
use tracing::log::info;

use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;
use crate::services::job::JobResponse;

#[derive(Debug, Deserialize)]
pub struct GetStatusParams {
    pub customer_id: String,
    pub cairo_job_key: String,
}

#[instrument(level = "info", skip(app_state))]
pub async fn get_status(
    State(app_state): State<AppState>,
    Query(params): Query<GetStatusParams>,
) -> Result<JsonResponse<JobResponse>, AppError> {
    let res = app_state.job_service.get_job_status(params).await?;

    Ok(JsonResponse(res))
}

#[instrument(level = "info", skip(_app_state))]
pub async fn get_proof(
    State(_app_state): State<AppState>,
    Query(params): Query<GetStatusParams>,
) -> Result<JsonResponse<Value>, AppError> {
    // TODO: process get proof
    info!("params: {:?}", params);
    let res = json!({
        "code" : "NO_OFFCHAIN_PROOF_FOR_JOB"
    });
    Ok(JsonResponse(res))
}
