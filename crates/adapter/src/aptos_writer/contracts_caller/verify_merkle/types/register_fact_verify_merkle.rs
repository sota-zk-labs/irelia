use aptos_sdk::move_types::u256::U256;
use aptos_sdk::rest_client::aptos_api_types::Event;

use crate::contracts_caller::transaction_helper::{str_to_u256, str_to_u64};
use crate::error::CoreError;
use crate::error::CoreError::PropertyNotFound;

#[derive(Debug)]
pub struct RegisterFactVerifyMerkle {
    pub channel_ptr: u64,
    pub data_to_hash_ptr: u64,
    pub n_queries: u64,
    pub res_root: U256,
}

impl TryInto<RegisterFactVerifyMerkle> for Event {
    type Error = CoreError;

    fn try_into(self) -> Result<RegisterFactVerifyMerkle, Self::Error> {
        Ok(RegisterFactVerifyMerkle {
            channel_ptr: str_to_u64(
                self.data
                    .get("channel_ptr")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
            data_to_hash_ptr: str_to_u64(
                self.data
                    .get("data_to_hash_ptr")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
            n_queries: str_to_u64(
                self.data
                    .get("n_queries")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
            res_root: str_to_u256(
                self.data
                    .get("res_root")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
        })
    }
}
