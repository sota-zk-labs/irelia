use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;
use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};
use tracing::instrument;

// #[instrument(level = "info", skip(app_state))]
// pub async fn get_status(
//     State(app_state): State<AppState>,
//     Query(params): Query<GetJobStatusQueryParams>
// ) -> Result<JsonResponse<GetJobStatusResponse>, AppError> {
//     todo!()
// }
//
// #[derive(Deserialize, Serialize, Debug)]
// pub struct GetJobStatusQueryParams {
//     customer_id: String,
//     cairo_job_key: String
// }
//
// #[derive(Clone, Debug, Deserialize, PartialEq)]
// pub struct GetJobStatusResponse {
//     status: CairoJobStatus,
//     invalid_reason: Option<InvalidReason>,
//     error_log: Option<String>,
//     validation_done: Option<bool>,
// }