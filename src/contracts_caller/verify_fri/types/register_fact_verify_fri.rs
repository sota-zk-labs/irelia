use aptos_sdk::move_types::u256::U256;
use aptos_sdk::rest_client::aptos_api_types::Event;

use crate::contracts_caller::transaction_helper::str_to_u256;
use crate::error::CoreError;
use crate::error::CoreError::PropertyNotFound;

#[derive(Debug)]
pub struct RegisterFactVerifyFri {
    pub data_to_hash: U256,
    pub fri_queue_ptr: U256,
}

impl TryInto<RegisterFactVerifyFri> for Event {
    type Error = CoreError;

    fn try_into(self) -> Result<RegisterFactVerifyFri, Self::Error> {
        Ok(RegisterFactVerifyFri {
            data_to_hash: str_to_u256(self.data.get("data_to_hash").ok_or(PropertyNotFound)?.as_str().unwrap())?,
            fri_queue_ptr: str_to_u256(self.data.get("fri_queue_ptr").ok_or(PropertyNotFound)?.as_str().unwrap())?,
        })
    }
}