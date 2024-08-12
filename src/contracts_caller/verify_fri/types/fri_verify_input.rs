use aptos_sdk::move_types::value::MoveValue;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriVerifyInput {
    pub proof: Vec<String>,
    pub fri_queue: Vec<String>,
    pub evaluation_point: String,
    pub fri_step_size: String,
    pub expected_root: String,
}

#[derive(Clone)]
pub struct VerifyFriTransactionInput {
    pub proof: MoveValue,
    pub fri_queue: MoveValue,
    pub evaluation_point: MoveValue,
    pub fri_step_size: MoveValue,
    pub expected_root: MoveValue,
}