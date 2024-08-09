use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;
use aptos_sdk::rest_client::aptos_api_types::Event;
use serde::Deserialize;
use crate::contracts_caller::transaction_helper::str_to_u256;
use crate::error::CoreError;
use crate::error::CoreError::PropertyNotFound;

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


#[derive(Debug)]
pub struct RegisterFactVerifyMerkle{
    pub channel_ptr: U256,
    pub data_to_hash_ptr: U256,
    pub n_queries: U256,
    pub res_root: U256,
}

impl TryInto<RegisterFactVerifyMerkle> for Event {
    type Error = CoreError;

    fn try_into(self) -> Result<RegisterFactVerifyMerkle, Self::Error> {
        Ok(RegisterFactVerifyMerkle {
            channel_ptr: str_to_u256(self.data.get("channel_ptr").ok_or(PropertyNotFound)?.as_str().unwrap())?,
            data_to_hash_ptr: str_to_u256(self.data.get("data_to_hash_ptr").ok_or(PropertyNotFound)?.as_str().unwrap())?,
            n_queries: str_to_u256(self.data.get("n_queries").ok_or(PropertyNotFound)?.as_str().unwrap())?,
            res_root: str_to_u256(self.data.get("res_root").ok_or(PropertyNotFound)?.as_str().unwrap())?,
        })
    }
}