use irelia_core::entities::job::{JobEntity, JobStatus};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

pub fn failed(job: JobEntity) -> JobResponse {
    JobResponse {
        status: job.status.to_string(),
        invalid_reason: None,
        error_log: Some("Sharp task failed".to_string()),
        validation_done: None,
    }
}

pub fn invalid(job: JobEntity) -> JobResponse {
    JobResponse {
        status: job.status.to_string(),
        invalid_reason: Some("INVALID_CAIRO_PIE_FILE_FORMAT".to_string()),
        error_log: Some(
            "The Cairo PIE file has a wrong format. \
                        Deserialization ended with \
                        exception: Invalid prefix for zip file.."
                .to_string(),
        ),
        validation_done: None,
    }
}

pub fn unknown(_: JobEntity) -> JobResponse {
    JobResponse {
        status: "FAILED".to_string(),
        invalid_reason: None,
        error_log: None,
        validation_done: None,
    }
}

pub fn in_progress(job: JobEntity) -> JobResponse {
    JobResponse {
        status: job.status.to_string(),
        invalid_reason: None,
        error_log: None,
        validation_done: Some(job.validation_done),
    }
}

pub fn not_created(job: JobEntity) -> JobResponse {
    JobResponse {
        status: job.status.to_string(),
        invalid_reason: None,
        error_log: None,
        validation_done: Some(job.validation_done),
    }
}

pub fn processed(job: JobEntity) -> JobResponse {
    JobResponse {
        status: job.status.to_string(),
        invalid_reason: None,
        error_log: None,
        validation_done: Some(job.validation_done),
    }
}

pub fn onchain(job: JobEntity) -> JobResponse {
    JobResponse {
        status: job.status.to_string(),
        invalid_reason: None,
        error_log: None,
        validation_done: Some(job.validation_done),
    }
}

pub fn get_job_response(job: JobEntity) -> JobResponse {
    match job.status {
        JobStatus::Failed => failed(job),
        JobStatus::Invalid => invalid(job),
        JobStatus::Unknown => unknown(job),
        JobStatus::InProgress => in_progress(job),
        JobStatus::NotCreated => not_created(job),
        JobStatus::Processed => processed(job),
        JobStatus::Onchain => onchain(job),
    }
}
