use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// Identifier for a question.
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct JobId(pub Uuid);

pub struct JobEntity {
    pub id: JobId,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub offchain_proof: bool,
    pub proof_layout: String,
    pub cairo_pie: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JobResponse {
    pub code: Option<String>,
    pub message: Option<String>,
}
