use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum JobStatus {
    Failed, // Stone failed
    Invalid, // Wrong pie format
    Unknown, //
    InProgress, // init status
    NotCreated,
    Processed, // stone completed => to submit on chain
    Onchain,  // stone completed and submit on chain completed
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobStatus::Failed => write!(f, "FAILED"),
            JobStatus::Invalid => write!(f, "INVALID"),
            JobStatus::Unknown => write!(f, "UNKNOWN"),
            JobStatus::InProgress => write!(f, "IN_PROGRESS"),
            JobStatus::NotCreated => write!(f, "NOT_CREATED"),
            JobStatus::Processed => write!(f, "PROCESSED"),
            JobStatus::Onchain => write!(f, "ONCHAIN"),
        }
    }
}

impl FromStr for JobStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "FAILED" => Ok(JobStatus::Failed),
            "INVALID" => Ok(JobStatus::Invalid),
            "UNKNOWN" => Ok(JobStatus::Unknown),
            "IN_PROGRESS" => Ok(JobStatus::InProgress),
            "NOT_CREATED" => Ok(JobStatus::NotCreated),
            "PROCESSED" => Ok(JobStatus::Processed),
            "ONCHAIN" => Ok(JobStatus::Onchain),
            _ => Err(format!("'{}' is not a valid value of job status", s)),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub struct JobId(pub Uuid);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct JobEntity {
    pub id: JobId,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub status: JobStatus,
    pub validation_done: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct JobEntityPayload {
    pub customer_id: String,
    pub cairo_job_key: String,
    pub status: JobStatus,
    pub validation_done: bool,
}