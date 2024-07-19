use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;

pub struct Verify {
    pub proof: MoveValue,
    pub fri_queue: MoveValue,
    pub evaluation_point: MoveValue,
    pub fri_step_size: MoveValue,
    pub expected_root: MoveValue,
}

pub struct InitFriGroup {
    fri_ctx: U256,
}

pub struct ComputeNextLayer {}