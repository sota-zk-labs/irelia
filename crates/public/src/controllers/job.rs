use std::str;

use axum::body::Bytes;
use irelia_core::entities::job::JobResponse;
use tracing::instrument;
use tracing::log::info;

use crate::errors::AppError;
use crate::json_response::JsonResponse;

#[instrument(level = "info")]
pub async fn add_job(body: Bytes) -> Result<JsonResponse<Vec<JobResponse>>, AppError> {
    info!("{}", str::from_utf8(&body).unwrap());
    // let question_filter = QuestionFilter::try_from(query).map_err(AppError::from)?;
    Ok(JsonResponse(vec![JobResponse {
        code: Some("JOB_RECEIVED_SUCCESSFULLY".to_string()),
        message: None,
    }]))
}
