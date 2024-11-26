use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// Identifier for a question.
#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub struct WorkerJobId(pub Uuid);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WorkerJob {
    pub id: WorkerJobId,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub offchain_proof: bool,
    pub proof_layout: String,
    pub cairo_pie: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NewWorkerJob {
    pub customer_id: String,
    pub cairo_job_key: String,
    pub offchain_proof: bool,
    pub proof_layout: String,
    pub cairo_pie: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WorkerJobResponse {
    pub code: Option<String>,
    pub message: Option<String>,
}