use std::fs::File;
use std::io::Write;

use ethers::{
    types::U256,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Adapted from https://github.com/zksecurity/stark-evm-adapter/blob/main/src/fri_merkle_statement.rs

/// Decommitment for a FRI layer merkle statement
#[derive(Serialize, Deserialize, Debug)]
pub struct FRIMerkleStatement {
    pub expected_root: U256,
    pub evaluation_point: U256,
    pub fri_step_size: usize,
    pub input_layer_queries: Vec<U256>,
    pub output_layer_queries: Vec<U256>,
    pub input_layer_values: Vec<U256>,
    pub output_layer_values: Vec<U256>,
    pub input_layer_inverses: Vec<U256>,
    pub output_layer_inverses: Vec<U256>,
    pub input_interleaved: Vec<U256>,
    pub output_interleaved: Vec<U256>,
    pub proof: Vec<U256>,
}

impl FRIMerkleStatement {

    pub fn to_json(&self) -> String {
        let mut fri_queue = self.input_interleaved.clone();
        fri_queue.push(U256::from(0));

        let json_data = json!({
            "expectedRoot": self.expected_root.to_string(),
            "friStepSize": self.fri_step_size.to_string(),
            "evaluationPoint": self.evaluation_point.to_string(),
            "friQueue": fri_queue.  iter().map(|p| p.to_string()).collect::<Vec<String>>(),
            "proof": self.proof.iter().map(|p| p.to_string()).collect::<Vec<String>>(),
        });
        serde_json::to_string_pretty(&json_data).expect("Unable to serialize data")
    }

    pub fn write_to_json(&self, file_name: &str) {
        let file_path = format!("{}.json", file_name);
        let mut file = File::create(file_path).expect("Unable to create file");
        let json_string = self.to_json();
        file.write_all(json_string.as_bytes()).expect("Unable to write data");
    }
}