use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum CairoJobStatus {
    FAILED,             // Stone failed
    INVALID,            // Wrong pie format
    UNKNOWN,            //
    IN_PROGRESS,        // init status
    NOT_CREATED,        //
    PROCESSED,          // stone completed => to submit on chain
    ONCHAIN,            // stone completed and submit on chain completed
}

impl fmt::Display for CairoJobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CairoJobStatus::FAILED => write!(f, "FAILED"),
            CairoJobStatus::INVALID => write!(f, "INVALID"),
            CairoJobStatus::UNKNOWN => write!(f, "UNKNOWN"),
            CairoJobStatus::IN_PROGRESS => write!(f, "IN_PROGRESS"),
            CairoJobStatus::NOT_CREATED => write!(f, "NOT_CREATED"),
            CairoJobStatus::PROCESSED => write!(f, "PROCESSED"),
            CairoJobStatus::ONCHAIN => write!(f, "ONCHAIN"),
        }
    }
}

impl FromStr for CairoJobStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "FAILED" => Ok(CairoJobStatus::FAILED),
            "INVALID" => Ok(CairoJobStatus::INVALID),
            "UNKNOWN" => Ok(CairoJobStatus::UNKNOWN),
            "IN_PROGRESS" => Ok(CairoJobStatus::IN_PROGRESS),
            "NOT_CREATED" => Ok(CairoJobStatus::NOT_CREATED),
            "PROCESSED" => Ok(CairoJobStatus::PROCESSED),
            "ONCHAIN" => Ok(CairoJobStatus::ONCHAIN),
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
    pub status: CairoJobStatus,
    pub invalid_reason: String,
    pub error_log: String,
    pub validation_done: bool,
}
