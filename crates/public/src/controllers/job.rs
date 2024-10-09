use std::str;

use axum::body::Bytes;
use rust_core::entities::job::JobResponse;
use tracing::instrument;
use tracing::log::info;

use crate::errors::AppError;
use crate::json_response::JsonResponse;

/// Handler for retrieving questions based on query parameters.
///
/// This function retrieves questions based on the provided query parameters. It takes a HashMap
/// containing the query parameters and a reference to the QuestionPort trait object. It returns
/// a JSON response containing the list of questions.
#[instrument(level = "info")]
pub async fn add_job(body: Bytes) -> Result<JsonResponse<Vec<JobResponse>>, AppError> {
    info!("{}", str::from_utf8(&body).unwrap());
    // let question_filter = QuestionFilter::try_from(query).map_err(AppError::from)?;
    Ok(JsonResponse(vec![JobResponse {
        code: Some("JOB_RECEIVED_SUCCESSFULLY".to_string()),
        message: None,
    }]))
}
