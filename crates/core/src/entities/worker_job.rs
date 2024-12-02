use std::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SUCCESSFULLY_CODE: &str = "JOB_RECEIVED_SUCCESSFULLY";
const INTERNAL_SERVER_ERROR_CODE: &str = "500";
const INTERNAL_SERVER_ERROR_MESSAGE: &str = "Internal server error";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ProofLayout {
    Plain,
    Small,
    Dex,
    Recursive,
    Starknet,
    StarknetWithKeccak,
    RecursiveLargeOutput,
    RecursiveWithPoseidon,
    AllSolidity,
    AllCairo,
    Dynamic,
}

impl Display for ProofLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for ProofLayout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(ProofLayout::Plain),
            "small" => Ok(ProofLayout::Small),
            "dex" => Ok(ProofLayout::Dex),
            "recursive" => Ok(ProofLayout::Recursive),
            "starknet" => Ok(ProofLayout::Starknet),
            "starknet_with_keccak" => Ok(ProofLayout::StarknetWithKeccak),
            "recursive_large_output" => Ok(ProofLayout::RecursiveLargeOutput),
            "recursive_with_poseidon" => Ok(ProofLayout::RecursiveWithPoseidon),
            "all_solidity" => Ok(ProofLayout::AllSolidity),
            "all_cairo" => Ok(ProofLayout::AllCairo),
            "dynamic" => Ok(ProofLayout::Dynamic),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorkerJobStatus {
    FaultyCairoPie,
    IncorrectLayout,
    AdditionalBadFlag,
    NoCairoJobId,
    IncorrectOffchainProof,

    Successfully,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub struct WorkerJobId(pub Uuid);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WorkerJobEntity {
    pub id: WorkerJobId,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub offchain_proof: bool,
    pub proof_layout: String,
    pub cairo_pie: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WorkerJobResponse {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl WorkerJobResponse {
    pub fn successfully() -> Self {
        WorkerJobResponse {
            code: SUCCESSFULLY_CODE.to_string(),
            message: None,
        }
    }

    pub fn internal_server_error() -> Self {
        WorkerJobResponse {
            code: INTERNAL_SERVER_ERROR_CODE.to_string(),
            message: Some(INTERNAL_SERVER_ERROR_MESSAGE.to_string()),
        }
    }

    pub fn get_worker_job_response(code: WorkerJobStatus) -> Self {
        match code {
            WorkerJobStatus::FaultyCairoPie => Self::successfully(),
            WorkerJobStatus::IncorrectLayout => Self::internal_server_error(),
            WorkerJobStatus::AdditionalBadFlag => Self::successfully(),
            WorkerJobStatus::NoCairoJobId => Self::internal_server_error(),
            WorkerJobStatus::IncorrectOffchainProof => Self::internal_server_error(),

            WorkerJobStatus::Successfully => Self::successfully(),
        }
    }
}
