use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Worker<T> {
    pub data: T,
    pub tracing: HashMap<String, String>,
}

pub const ADD_JOB_WORKER_IDENTIFIER: &str = "add_job";
pub const MERKLE_STATEMENT_VERIFIER_IDENTIFIER: &str = "merkle_statement_verifier";
pub const VERIFY_FRI_IDENTIFIER: &str = "verify_fri";
pub const REGISTER_CONTINUOUS_IDENTIFIER: &str = "register_continuous";
pub const VERIFY_PROOF_AND_REGISTER_IDENTIFIER: &str = "verify_proof_and_register";
