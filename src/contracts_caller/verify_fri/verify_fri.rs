use std::str::FromStr;

use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;

use crate::config::AppConfig;
use crate::contracts_caller::types::VerifyMerkle;
use crate::contracts_caller::verify_fri::compute_next_layer::compute_next_layer;
use crate::contracts_caller::verify_fri::fri_statement::fri_statement;
use crate::contracts_caller::verify_fri::init_fri_group::init_fri_group;
use crate::contracts_caller::verify_fri::merkle_verifier::merkle_verifier;
use crate::contracts_caller::verify_fri::register_fact_fri::register_fact_fri;
use crate::contracts_caller::verify_fri::types::compute_next_layer::ComputeNextLayer;
use crate::contracts_caller::verify_fri::types::fri_verify_input::{FriVerifyInput, VerifyFriTransactionInput};
use crate::contracts_caller::verify_fri::types::init_fri_group::InitFriGroup;
use crate::contracts_caller::verify_fri::types::register_fact_verify_fri::RegisterFactVerifyFri;

pub async fn verify_fri(
    config: &AppConfig,
    fri_verify_input: FriVerifyInput,
    proof: MoveValue, fri_queue: MoveValue,
    evaluation_point: MoveValue,
    fri_step_size: MoveValue,
    expected_root: MoveValue,
) -> anyhow::Result<()> {
    let verify_merkle_input = VerifyFriTransactionInput {
        proof,
        fri_queue,
        evaluation_point,
        fri_step_size,
        expected_root,
    };

    let (event_init, event_compute, event_register) = fri_statement(&config, verify_merkle_input.clone()).await?;

    let input_init: InitFriGroup = event_init.try_into()?;
    let input_compute: ComputeNextLayer = event_compute.clone().try_into()?;
    let input_register: RegisterFactVerifyFri = event_register.try_into()?;

    if !init_fri_group(&config, input_init).await? {
        eprintln!("something went wrong!");
        return Ok(());
    }

    if !compute_next_layer(&config, &input_compute).await? {
        eprintln!("something went wrong!");
        return Ok(());
    }

    let input_verify_merkle: VerifyMerkle = VerifyMerkle {
        channel_ptr: input_compute.channel_ptr,
        merkle_queue_ptr: input_compute.merkle_queue_ptr,
        expected_root: U256::from_str(&*fri_verify_input.expected_root).unwrap(),
        n_queries: input_compute.n_queries,
    };

    if !merkle_verifier(&config, &input_verify_merkle).await? {
        eprintln!("something went wrong!");
        return Ok(());
    }

    if !register_fact_fri(&config, input_register, input_compute.n_queries).await? {
        eprintln!("something went wrong!");
        return Ok(());
    }

    Ok(())
}

