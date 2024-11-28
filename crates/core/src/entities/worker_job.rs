use std::fmt;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// Identifier for a question.

#[derive(Clone, Debug, PartialEq)]
pub enum WorkerJobStatus {
    FaultyCairoPie,
    IncorrectLayout,
    AdditionalBadFlag,
    NoCairoJobId,
    IncorrectOffchainProof,

    Successfully
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub struct WorkerJobId(pub Uuid);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WorkerJob {
    pub id: WorkerJobId,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub offchain_proof: bool,
    pub proof_layout: String,
    pub cairo_pie: String,
}