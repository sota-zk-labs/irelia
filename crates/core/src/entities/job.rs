use std::str::FromStr;

use serde::{Deserialize, Serialize};
use starknet_os::sharp::CairoJobStatus;
use uuid::Uuid;

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
