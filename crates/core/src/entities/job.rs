use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum JobStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub struct JobId(pub Uuid);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Job {
    pub id: JobId,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub status: String,
    pub validation_done: bool,
}
impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobStatus::Pending => write!(f, "PENDING"),
            JobStatus::InProgress => write!(f, "IN_PROGRESS"),
            JobStatus::Completed => write!(f, "COMPLETED"),
            JobStatus::Failed => write!(f, "FAILED"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct JobResponse {
    pub status: Option<String>,
    pub validation: Option<String>,
}
