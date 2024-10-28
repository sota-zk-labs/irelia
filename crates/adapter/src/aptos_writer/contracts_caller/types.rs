use aptos_sdk::move_types::u256::U256;
use aptos_sdk::rest_client::aptos_api_types::Event;
use irelia_core::common::aptos_writer_error::AptosWriterError;
use irelia_core::common::aptos_writer_error::AptosWriterError::PropertyNotFound;

use crate::aptos_writer::contracts_caller::transaction_helper::{str_to_u256, str_to_u64};

#[derive(Debug)]
pub struct VerifyMerkle {
    pub channel_ptr: u64,
    pub merkle_queue_ptr: u64,
    pub expected_root: U256,
    pub n_queries: u64,
}

impl TryInto<VerifyMerkle> for Event {
    type Error = AptosWriterError;

    fn try_into(self) -> Result<VerifyMerkle, Self::Error> {
        Ok(VerifyMerkle {
            channel_ptr: str_to_u64(
                self.data
                    .get("channel_ptr")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
            merkle_queue_ptr: str_to_u64(
                self.data
                    .get("merkle_queue_ptr")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
            expected_root: str_to_u256(
                self.data
                    .get("expected_root")
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
        })
    }
}
