use aptos_sdk::rest_client::aptos_api_types::Event;
use rust_core::common::aptos_writer_error::AptosWriterError;
use rust_core::common::aptos_writer_error::AptosWriterError::PropertyNotFound;

use crate::aptos_writer::contracts_caller::transaction_helper::str_to_u64;

#[derive(Debug)]
pub struct InitFriGroup {
    pub fri_ctx: u64,
}

impl TryInto<InitFriGroup> for Event {
    type Error = AptosWriterError;

    fn try_into(self) -> Result<InitFriGroup, Self::Error> {
        Ok(InitFriGroup {
            fri_ctx: str_to_u64(
                self.data
                    .get("fri_ctx")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
        })
    }
}
