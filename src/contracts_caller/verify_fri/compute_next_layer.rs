use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::types::transaction::{EntryFunction, SignedTransaction, TransactionPayload};
use tokio::task::JoinHandle;

use crate::config::AppConfig;
use crate::contracts_caller::transaction_helper::{build_simulated_transaction, build_transaction};
use crate::contracts_caller::verify_fri::types::compute_next_layer::ComputeNextLayer;
use crate::contracts_caller::vm_status::VmStatus;

const ECOMPUTE_NEXT_LAYER_NOT_INITIATED: &str = "ECOMPUTE_NEXT_LAYER_NOT_INITIATED";

pub async fn compute_next_layer(loop_cycles: usize, config: &AppConfig, data: &ComputeNextLayer) -> anyhow::Result<()> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_layer").unwrap()),
            Identifier::new("compute_next_layer").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    MoveValue::U256(data.channel_ptr.clone()),
                    MoveValue::U256(data.fri_queue_ptr.clone()),
                    MoveValue::U256(data.merkle_queue_ptr.clone()),
                    MoveValue::U256(data.n_queries.clone()),
                    MoveValue::U256(data.fri_ctx.clone()),
                    MoveValue::U256(data.evaluation_point.clone()),
                    MoveValue::U256(data.fri_coset_size.clone()),
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let transaction_info = transaction.transaction_info().unwrap();
    println!("finished compute next layer {}; gas used: {}", transaction_info.hash.to_string(),
             transaction_info.gas_used);
    Ok(())
}

pub async fn simulate_compute_next_layer(config: &AppConfig, data: &ComputeNextLayer) -> anyhow::Result<bool> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_layer").unwrap()),
            Identifier::new("compute_next_layer").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    MoveValue::U256(data.channel_ptr.clone()),
                    MoveValue::U256(data.fri_queue_ptr.clone()),
                    MoveValue::U256(data.merkle_queue_ptr.clone()),
                    MoveValue::U256(data.n_queries.clone()),
                    MoveValue::U256(data.fri_ctx.clone()),
                    MoveValue::U256(data.evaluation_point.clone()),
                    MoveValue::U256(data.fri_coset_size.clone()),
                ]
            ),
        ));
    let tx = build_simulated_transaction(payload, &config.account, config.chain_id);
    let simulate = config.client.simulate(&tx).await?.into_inner();
    let vm_status = simulate.get(0).unwrap().info.vm_status.as_str();
    let vm_status: VmStatus = vm_status.try_into()?;
    if vm_status.reason == ECOMPUTE_NEXT_LAYER_NOT_INITIATED {
        return Ok(true);
    }
    Ok(false)
}