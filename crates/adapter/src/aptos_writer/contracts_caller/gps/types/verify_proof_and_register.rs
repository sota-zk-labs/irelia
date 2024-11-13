use std::str::FromStr;

use aptos_sdk::move_types::u256::U256;
use serde::Deserialize;

#[derive(Debug)]
pub struct VerifyProofAndRegisterData {
    pub proof_params: Vec<U256>,
    pub proof: Vec<U256>,
    pub task_metadata: Vec<U256>,
    pub cairo_aux_input: Vec<U256>,
    pub cairo_verifier_id: U256,
    pub pre_registered_facts: Option<Vec<U256>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyProofAndRegisterDataJson {
    pub proof_params: Vec<String>,
    pub proof: Vec<String>,
    pub task_metadata: Vec<String>,
    pub cairo_aux_input: Vec<String>,
    pub cairo_verifier_id: String,
    pub pre_registered_facts: Option<Vec<String>>,
}

impl From<VerifyProofAndRegisterDataJson> for VerifyProofAndRegisterData {
    fn from(value: VerifyProofAndRegisterDataJson) -> Self {
        VerifyProofAndRegisterData {
            proof_params: value
                .proof_params
                .iter()
                .map(|x| U256::from_str(x).unwrap())
                .collect(),
            proof: value
                .proof
                .iter()
                .map(|x| U256::from_str(x).unwrap())
                .collect(),
            task_metadata: value
                .task_metadata
                .iter()
                .map(|x| U256::from_str(x).unwrap())
                .collect(),
            cairo_aux_input: value
                .cairo_aux_input
                .iter()
                .map(|x| U256::from_str(x).unwrap())
                .collect(),
            cairo_verifier_id: U256::from_str(value.cairo_verifier_id.as_str()).unwrap(),
            pre_registered_facts: value
                .pre_registered_facts
                .map(|data| data.iter().map(|x| U256::from_str(x).unwrap()).collect()),
        }
    }
}
