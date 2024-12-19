use ethers::prelude::U256;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
}
