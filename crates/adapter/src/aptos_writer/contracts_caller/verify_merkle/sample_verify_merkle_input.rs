use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;
use irelia_core::entities::merkle_statement::VerifyMerkleTransactionInput;

use crate::aptos_writer::contracts_caller::verify_merkle::types::verify_merkle_input::MerkleVerifyInput;

pub fn sample_verify_merkle_input(index: isize) -> anyhow::Result<VerifyMerkleTransactionInput> {
    let file_path = format!(
        "./src/test_samples/test_samples/merkle_verify/merkle_verify_{}.json",
        index
    );
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let merkle_verify_input: MerkleVerifyInput = serde_json::from_reader(reader)?;

    let mut merkle_view_vec = vec![];
    for i in 0..merkle_verify_input.merkle_view.len() {
        merkle_view_vec.push(MoveValue::U256(U256::from_str(
            &merkle_verify_input.merkle_view[i].clone(),
        )?));
    }
    let merkle_view = MoveValue::Vector(merkle_view_vec);

    let mut initial_merkle_queue_vec = vec![];
    for i in 0..merkle_verify_input.initial_merkle_queue.len() {
        initial_merkle_queue_vec.push(MoveValue::U256(U256::from_str(
            &merkle_verify_input.initial_merkle_queue[i],
        )?));
    }
    let initial_merkle_queue = MoveValue::Vector(initial_merkle_queue_vec);

    let height = MoveValue::U64(u64::from_str(&merkle_verify_input.height.clone())?);
    let expected_root =
        MoveValue::U256(U256::from_str(&merkle_verify_input.expected_root.clone())?);
    Ok(VerifyMerkleTransactionInput {
        merkle_view,
        initial_merkle_queue,
        height,
        expected_root,
    })
}
