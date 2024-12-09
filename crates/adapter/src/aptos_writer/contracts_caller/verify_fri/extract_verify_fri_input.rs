use std::str::FromStr;

use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;

use crate::aptos_writer::contracts_caller::verify_fri::types::fri_verify_input::FriVerifyInput;

pub fn extract_verify_fri_input(
    fri_inputs: &[String],
) -> anyhow::Result<
    Vec<(
        FriVerifyInput,
        MoveValue,
        MoveValue,
        MoveValue,
        MoveValue,
        MoveValue,
    )>,
> {
    let mut res: Vec<(
        FriVerifyInput,
        MoveValue,
        MoveValue,
        MoveValue,
        MoveValue,
        MoveValue,
    )> = vec![];
    for fri_input in fri_inputs {
        let fri_verify_input: FriVerifyInput = serde_json::from_str(fri_input)?;

        //proof
        let mut proof_vec = vec![];
        for i in 0..fri_verify_input.proof.len() {
            proof_vec.push(MoveValue::U256(U256::from_str(
                &fri_verify_input.proof[i].clone(),
            )?));
        }
        let proof = MoveValue::Vector(proof_vec);

        //queue
        let mut fri_queue_vec = vec![];
        for i in 0..fri_verify_input.fri_queue.len() {
            fri_queue_vec.push(MoveValue::U256(U256::from_str(
                &fri_verify_input.fri_queue[i].clone(),
            )?));
        }
        let fri_queue = MoveValue::Vector(fri_queue_vec);

        let evaluation_point =
            MoveValue::U256(U256::from_str(&fri_verify_input.evaluation_point.clone())?);
        let fri_step_size =
            MoveValue::U256(U256::from_str(&fri_verify_input.fri_step_size.clone())?);
        let expected_root =
            MoveValue::U256(U256::from_str(&fri_verify_input.expected_root.clone())?);
        res.push((
            fri_verify_input,
            proof,
            fri_queue,
            evaluation_point,
            fri_step_size,
            expected_root,
        ));
    }
    Ok(res)
}
