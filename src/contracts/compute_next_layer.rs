use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts::helper::{build_simulated_transaction, build_transaction};
use crate::contracts::types::ComputeNextLayer;
use crate::contracts::vm_status::VmStatus;

pub async fn compute_next_layer(loop_cycles: usize, config: &AppConfig, data: &ComputeNextLayer) -> Result<(), ()> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_layer").unwrap()),
            Identifier::new("init_compute_next_layer").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    MoveValue::U256(data.fri_queue_ptr.clone()),
                    MoveValue::U256(data.merkle_queue_ptr.clone()),
                    MoveValue::U256(data.n_queries.clone()),
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let init_transaction = config.client.submit(&tx).await.unwrap().into_inner();
    eprintln!("sent init_compute_next_layer = {:#?}", init_transaction.hash.to_string());
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_layer").unwrap()),
            Identifier::new("compute_next_layer").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    MoveValue::U256(data.channel_ptr.clone()),
                    MoveValue::U256(data.fri_ctx.clone()),
                    MoveValue::U256(data.evaluation_point.clone()),
                    MoveValue::U256(data.fri_coset_size.clone()),
                ]
            ),
        ));

    let mut txs = vec![];
    for i in 0..loop_cycles {
        let tx = build_transaction(payload.clone(), &config.account, config.chain_id);
        let transaction = config.client.submit(&tx).await.unwrap().into_inner();
        eprintln!("sent compute next layer {}: {:#?}", i, transaction.hash.to_string());
        txs.push(transaction);
    }

    let init_transaction = config.client.wait_for_transaction(&init_transaction).await.unwrap().into_inner();
    eprintln!("finished init_compute_next_layer: {:#?}", init_transaction.transaction_info().unwrap().hash.to_string());

    for i in 0..loop_cycles {
        let transaction = config.client.wait_for_transaction(txs.get(i).unwrap()).await.unwrap().into_inner();
        eprintln!("finished next layer {}: {:#?}", i, transaction.transaction_info().unwrap().hash.to_string());
    }
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
    if vm_status.reason == "ECOMPUTE_NEXT_LAYER_NOT_INITIATED" {
        println!("compute_next_layer check passes");
        return Ok(true);
    }
    Ok(false)
}