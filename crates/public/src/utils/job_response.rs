use std::str::FromStr;
use serde::{Deserialize, Serialize};
use irelia_core::entities::job::JobStatus;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JobResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_log: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_done: Option<bool>,
}


pub fn failed() -> JobResponse {
    JobResponse {
        status:"FAILED".to_string(),
        invalid_reason: None,
        error_log: Some("Sharp task failed".to_string()),
        validation_done: None,
    }
}

pub fn invalid() -> JobResponse {
    JobResponse {
        status:"INVALID".to_string(),
        invalid_reason: Some("INVALID_CAIRO_PIE_FILE_FORMAT".to_string()),
        error_log: Some("The Cairo PIE file has a wrong format. \
                        Deserialization ended with \
                        exception: Invalid prefix for zip file..".to_string()),
        validation_done: None,
    }
}

pub fn unknown() -> JobResponse {
    JobResponse {
        status:"FAILED".to_string(),
        invalid_reason: None,
        error_log: None,
        validation_done: None,
    }
}

pub fn in_progress() -> JobResponse {
    JobResponse {
        status:"IN_PROGRESS".to_string(),
        invalid_reason: None,
        error_log: None,
        validation_done: Some(false),
    }
}

pub fn not_created() -> JobResponse {
    JobResponse {
        status:"NOT_CREATED".to_string(),
        invalid_reason: None,
        error_log: None,
        validation_done: Some(false),
    }
}

pub fn processed() -> JobResponse {
    JobResponse {
        status:"PROCESSED".to_string(),
        invalid_reason: None,
        error_log: None,
        validation_done: Some(false),
    }
}


pub fn onchain() -> JobResponse {
    JobResponse {
        status:"ONCHAIN".to_string(),
        invalid_reason: None,
        error_log: None,
        validation_done: Some(true),
    }
}

pub fn get_job_response(status: JobStatus) -> JobResponse {
    match status {
        JobStatus::Failed => failed(),
        JobStatus::Invalid => invalid(),
        JobStatus::Unknown => unknown(),
        JobStatus::InProgress => in_progress(),
        JobStatus::NotCreated => not_created(),
        JobStatus::Processed => processed(),
        JobStatus::Onchain => onchain(),
    }
}