use ethers::prelude::U256;
use serde::{Deserialize, Serialize};

use crate::entities::annotated_proof::{ProofParameters, PublicInput};

#[derive(Serialize, Deserialize, Debug)]
pub struct MainProof {
    pub proof: Vec<U256>,
    pub proof_parameters: ProofParameters,
    pub public_input: PublicInput,
    pub interaction_z: U256,
    pub interaction_alpha: U256,
}

// todo use thiserror
impl MainProof {
    pub fn new(
        proof: Vec<U256>,
        proof_parameters: ProofParameters,
        public_input: PublicInput,
        interaction_z: U256,
        interaction_alpha: U256,
    ) -> MainProof {
        MainProof {
            proof,
            proof_parameters,
            public_input,
            interaction_z,
            interaction_alpha,
        }
    }
}
