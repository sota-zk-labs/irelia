use std::collections::HashMap;

use axum::extract::{Query, State};
use diesel::row::NamedRow;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, Queryable, RunQueryDsl, SelectableHelper,
};
use irelia_adapter::repositories::postgres::models::job::JobModel;
use irelia_adapter::repositories::postgres::schema::jobs::dsl::jobs;
use irelia_adapter::repositories::postgres::schema::worker_job::{cairo_job_key, customer_id};
use irelia_core::common::core_error::CoreError;
use irelia_core::entities::job::JobResponse;
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
) -> Result<JsonResponse<JobResponse>, AppError> {
    let db = app_state.db.clone();

    let job_entity = db
        .get()
        .await
        .unwrap()
        .interact(move |conn| {
            use irelia_adapter::repositories::postgres::schema::jobs::dsl::*;
            jobs
                .select(JobModel::as_select())
                .filter(customer_id.eq(params.customer_id))
                .filter(cairo_job_key.eq(params.cairo_job_key))
                .first::<JobModel>(conn)
                .optional()
        })
        .await
        .unwrap()
        .unwrap();
    Ok(JsonResponse(JobResponse {
        status: Some(job_entity.clone().unwrap().status.to_string()),
        validation: Some(job_entity.unwrap().validation_done.to_string()),
    }))
}
