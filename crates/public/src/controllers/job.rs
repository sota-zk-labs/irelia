use std::collections::HashMap;

use axum::extract::{Query, State};
use diesel::row::NamedRow;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, Queryable, RunQueryDsl, SelectableHelper,
};
use irelia_adapter::repositories::postgres::models::job::JobModel;
use irelia_adapter::repositories::postgres::schema::jobs::dsl::jobs;
use irelia_core::common::core_error::CoreError;
use irelia_core::entities::job::{JobEntity, JobEntityResponse};
use serde::Deserialize;
use tracing::instrument;
use tracing::log::info;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;

#[derive(Debug, Deserialize)]
pub struct GetStatusParams {
    customer_id: String,
    cairo_job_key: String,
}

#[instrument(level = "info", skip(app_state))]
#[axum::debug_handler]
pub async fn get_status(
    State(app_state): State<AppState>,
    Query(params): Query<GetStatusParams>,
) -> Result<JsonResponse<JobEntityResponse>, AppError> {
    let job : JobEntity= app_state
        .job_port
        .get(params.customer_id, params.cairo_job_key)
        .await?;


    Ok(JsonResponse(JobEntityResponse {
        status: Some(job.status.to_string()),
        validation: Some(job.validation_done.to_string()),
    }))
}
