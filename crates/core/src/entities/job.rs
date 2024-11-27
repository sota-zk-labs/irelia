use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum JobStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
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

impl FromStr for JobStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PENDING" => Ok(JobStatus::Pending),
            "IN_PROGRESS" => Ok(JobStatus::InProgress),
            "COMPLETED" => Ok(JobStatus::Completed),
            "FAILED" => Ok(JobStatus::Failed),
            _ => Err(format!("'{}' is not a valid value of job status", s)),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub struct JobId(pub Uuid);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Job {
    pub id: JobId,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub status: JobStatus,
    pub validation_done: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct JobPayload {
    pub customer_id: String,
    pub cairo_job_key: String,
    pub status: JobStatus,
    pub validation_done: bool,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct JobResponse {
    pub status: Option<String>,
    pub validation: Option<String>,
}
