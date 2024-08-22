use aptos_sdk::move_types::value::MoveValue;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerkleVerifyInput {
    pub merkle_view: Vec<String>,
    pub initial_merkle_queue: Vec<String>,
    pub height: String,
    pub expected_root: String,
}

pub struct VerifyMerkleTransactionInput {
    pub merkle_view: MoveValue,
    pub initial_merkle_queue: MoveValue,
    pub height: MoveValue,
    pub expected_root: MoveValue,
}
