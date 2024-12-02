use std::collections::HashMap;
use std::str;

use axum::body::{Body, Bytes};
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::Json;
use irelia_core::entities::job::JobStatus::{InProgress, Invalid};
use irelia_core::entities::job::{JobEntity, JobId, JobStatus};
use irelia_core::entities::worker_job::WorkerJobStatus::{AdditionalBadFlag, FaultyCairoPie};
use irelia_core::entities::worker_job::{
    WorkerJobEntity, WorkerJobId, WorkerJobResponse, WorkerJobStatus,
};
use irelia_core::ports::job::JobPort;
use openssl::pkey::Params;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use tracing::instrument;
use tracing::log::info;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;

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
    info!("params: {:?}, req: {:?}", params, req);
    let res = app_state
        .worker_service
        .add_worker_job(app_state.job_service, params, req)
        .await?;

    Ok(JsonResponse(res))
}
