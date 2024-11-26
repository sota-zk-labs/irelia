use serde::{Deserialize, Serialize};

use crate::entities::job::JobEntity;
use crate::entities::sharp_proof::SharpProof;

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadVerifyJob {
    pub job: JobEntity,
    pub sharp_proof: SharpProof,
}
