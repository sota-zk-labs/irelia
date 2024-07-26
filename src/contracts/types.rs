use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;
use aptos_sdk::rest_client::aptos_api_types::Event;

use crate::contracts::helper::str_to_u256;
use crate::error::CoreError;
use crate::error::CoreError::PropertyNotFound;

pub struct VerifyTransactionInput {
    pub proof: MoveValue,
    pub fri_queue: MoveValue,
    pub evaluation_point: MoveValue,
    pub fri_step_size: MoveValue,
    pub expected_root: MoveValue,
}

#[derive(Debug)]
pub struct InitFriGroup {
    pub fri_ctx: MoveValue,
}

impl TryInto<InitFriGroup> for Event {
    type Error = CoreError;

    fn try_into(self) -> Result<InitFriGroup, Self::Error> {
        Ok(InitFriGroup {
            fri_ctx: MoveValue::U256(str_to_u256(self.data.get("fri_ctx").ok_or(PropertyNotFound)?.as_str().unwrap())?),
        })
    }
}


#[derive(Debug)]
pub struct ComputeNextLayer {
    pub channel_ptr: U256,
    pub fri_queue_ptr: U256,
    pub fri_ctx: U256,
    pub evaluation_point: U256,
    pub fri_coset_size: U256,
    pub merkle_queue_ptr: U256,
    pub n_queries: U256,
}

impl TryInto<ComputeNextLayer> for Event {
    type Error = CoreError;

    fn try_into(self) -> Result<ComputeNextLayer, Self::Error> {
        Ok(ComputeNextLayer {
            channel_ptr: str_to_u256(self.data.get("channel_ptr").ok_or(PropertyNotFound)?.as_str().unwrap())?,
            evaluation_point: str_to_u256(self.data.get("evaluation_point").ok_or(PropertyNotFound)?.as_str().unwrap())?,
            fri_coset_size: str_to_u256(self.data.get("fri_coset_size").ok_or(PropertyNotFound)?.as_str().unwrap())?,
            fri_ctx: str_to_u256(self.data.get("fri_ctx").ok_or(PropertyNotFound)?.as_str().unwrap())?,
            fri_queue_ptr: str_to_u256(self.data.get("fri_queue_ptr").ok_or(PropertyNotFound)?.as_str().unwrap())?,
            merkle_queue_ptr: str_to_u256(self.data.get("merkle_queue_ptr").ok_or(PropertyNotFound)?.as_str().unwrap())?,
            n_queries: str_to_u256(self.data.get("n_queries").ok_or(PropertyNotFound)?.as_str().unwrap())?,
        })
    }
}


