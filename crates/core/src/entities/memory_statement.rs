use ethers::prelude::U256;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContinuousMemoryPage {
    pub start_address: U256,
    pub values: Vec<U256>,
}

impl ContinuousMemoryPage {
    pub fn to_json(&self, z: U256, alpha: U256, prime: U256) -> String {
        let json_data = json!({
            "startAddr": self.start_address.to_string(),
            "values": self.values.iter().map(|v| v.to_string()).collect::<Vec<String>>(),
            "z": z.to_string(),
            "alpha": alpha.to_string(),
            "prime": prime.to_string(),
        });

        serde_json::to_string_pretty(&json_data).expect("Unable to serialize data")
    }
}
