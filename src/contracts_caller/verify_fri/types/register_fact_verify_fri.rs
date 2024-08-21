use aptos_sdk::rest_client::aptos_api_types::Event;

use crate::contracts_caller::transaction_helper::str_to_u64;
use crate::error::CoreError;
use crate::error::CoreError::PropertyNotFound;

#[derive(Debug)]
pub struct RegisterFactVerifyFri {
    pub data_to_hash: u64,
    pub fri_queue_ptr: u64,
}

impl TryInto<RegisterFactVerifyFri> for Event {
    type Error = CoreError;

    fn try_into(self) -> Result<RegisterFactVerifyFri, Self::Error> {
        Ok(RegisterFactVerifyFri {
            data_to_hash: str_to_u64(
                self.data
                    .get("data_to_hash")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
            fri_queue_ptr: str_to_u64(
                self.data
                    .get("fri_queue_ptr")
                    .ok_or(PropertyNotFound)?
                    .as_str()
                    .unwrap(),
            )?,
        })
    }
}
