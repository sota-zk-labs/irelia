use std::collections::HashMap;
use std::str;
use std::str::FromStr;
use std::sync::Arc;

use axum::body::Bytes;
use axum::http::StatusCode;
use axum::Json;
use rust_core::entities::question::{QuestionEntity, QuestionId};
use rust_core::entities::question_filter::QuestionFilter;
use rust_core::ports::question::QuestionPort;
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
pub async fn add_job(body: Bytes) -> Result<JsonResponse<Vec<QuestionEntity>>, AppError> {
    info!("{}", str::from_utf8(&body).unwrap());
    // let question_filter = QuestionFilter::try_from(query).map_err(AppError::from)?;
    Ok(JsonResponse(vec![QuestionEntity {
        id: QuestionId("1".to_string()),
        title: "1".to_string(),
        content: "32".to_string(),
        tags: None,
    }]))
}
