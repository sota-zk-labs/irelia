use aptos_sdk::move_types::u256::U256;
use aptos_sdk::rest_client::aptos_api_types::Event;

use crate::contracts_caller::transaction_helper::str_to_u256;
use crate::error::CoreError;
use crate::error::CoreError::PropertyNotFound;

#[derive(Debug)]
pub struct LogMemoryPageFactContinuous {
    pub fact_hash: U256,
    pub memory_hash: U256,
    pub prod: U256,
}

impl TryInto<LogMemoryPageFactContinuous> for Event {
    type Error = CoreError;

    fn try_into(self) -> Result<LogMemoryPageFactContinuous, Self::Error> {
        Ok(LogMemoryPageFactContinuous {
            fact_hash: str_to_u256(
                self.data
                    .get("channel_ptr")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
            memory_hash: str_to_u256(
                self.data
                    .get("data_to_hash_ptr")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
            prod: str_to_u256(
                self.data
                    .get("n_queries")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
        })
    }
}
