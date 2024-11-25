use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// Identifier for a question.
#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub struct JobId(pub Uuid);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct JobEntity {
    pub id: JobId,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub offchain_proof: bool,
    pub proof_layout: String,
    pub cairo_pie: String,
    // pub status: String,
    // pub invalid_reason: Option<String>,
    // pub error_log: Option<String>,
    // pub validation_done: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JobResponse {
    pub code: Option<String>,
    pub message: Option<String>,
}
