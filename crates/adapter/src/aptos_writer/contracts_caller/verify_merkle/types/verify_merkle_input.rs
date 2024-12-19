use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MerkleVerifyInput {
    pub merkle_view: Vec<String>,
    pub initial_merkle_queue: Vec<String>,
    pub height: String,
    pub expected_root: String,
}
