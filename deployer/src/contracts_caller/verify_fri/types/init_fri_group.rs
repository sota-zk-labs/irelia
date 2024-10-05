use aptos_sdk::rest_client::aptos_api_types::Event;

use crate::contracts_caller::transaction_helper::str_to_u64;
use crate::error::CoreError;
use crate::error::CoreError::PropertyNotFound;

#[derive(Debug)]
pub struct InitFriGroup {
    pub fri_ctx: u64,
}

impl TryInto<InitFriGroup> for Event {
    type Error = CoreError;

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
