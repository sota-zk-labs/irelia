use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;
use crate::utils::job_response::{get_job_response, JobResponse};
use axum::extract::{Query, State};
use irelia_core::entities::job::JobEntity;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::instrument;
use tracing::log::info;

#[derive(Debug, Deserialize)]
pub struct GetStatusParams {
    customer_id: String,
    cairo_job_key: String,
}

#[instrument(level = "info", skip(app_state))]
pub async fn get_status(
    State(app_state): State<AppState>,
    Query(params): Query<GetStatusParams>,
) -> Result<JsonResponse<JobResponse>, AppError> {
    info!("params: {:?}", params);
    let job: JobEntity = app_state
        .job_port
        .get(params.customer_id, params.cairo_job_key)
        .await?;

    Ok(JsonResponse(get_job_response(job)))
}

#[instrument(level = "info", skip(app_state))]
pub async fn get_proof(
    State(app_state): State<AppState>,
    Query(params): Query<GetStatusParams>,
) -> Result<JsonResponse<Value>, AppError> {
    info!("params: {:?}", params);
    let res = json!({
        "code" : "NO_OFFCHAIN_PROOF_FOR_JOB"
    });
    Ok(JsonResponse(res))
}
