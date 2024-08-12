use aptos_sdk::move_types::value::MoveValue;
use aptos_sdk::rest_client::aptos_api_types::Event;

use crate::contracts_caller::transaction_helper::str_to_u256;
use crate::error::CoreError;
use crate::error::CoreError::PropertyNotFound;

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