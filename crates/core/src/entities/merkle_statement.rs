use aptos_sdk::move_types::value::MoveValue;
use ethers::prelude::U256;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MerkleStatement {
    pub expected_root: U256,
    pub n_unique_queries: usize,
    pub merkle_height: usize,
    pub merkle_queue_indices: Vec<U256>,
    pub merkle_queue_values: Vec<U256>,
    pub proof: Vec<U256>,
}

#[derive(Serialize, Debug)]
pub struct VerifyMerkleTransactionInput {
    pub merkle_view: MoveValue,
    pub initial_merkle_queue: MoveValue,
    pub height: MoveValue,
    pub expected_root: MoveValue,
}

impl MerkleStatement {
    pub fn new(
        expected_root: U256,
        n_unique_queries: usize,
        merkle_height: usize,
        merkle_queue_indices: Vec<U256>,
        merkle_queue_values: Vec<U256>,
        proof: Vec<U256>,
    ) -> MerkleStatement {
        MerkleStatement {
            expected_root,
            n_unique_queries,
            merkle_height,
            merkle_queue_indices,
            merkle_queue_values,
            proof,
        }
    }

    pub fn to_json(&self) -> String {
        // Constructs the merkle_queue by interleaving indices and values.
        let initial_merkle_queue: Vec<String> = self
            .merkle_queue_indices
            .iter()
            .zip(self.merkle_queue_values.iter())
            .flat_map(|(&index, &value)| vec![index.to_string(), value.to_string()])
            .collect();

        let json_data = json!({
            "expectedRoot": self.expected_root.to_string(),
            "height": self.merkle_height.to_string(),
            "merkleView": self.proof.iter().map(|p| p.to_string()).collect::<Vec<String>>(),
            "initialMerkleQueue": initial_merkle_queue,
        });

        serde_json::to_string_pretty(&json_data).expect("Unable to serialize data")
    }
}
