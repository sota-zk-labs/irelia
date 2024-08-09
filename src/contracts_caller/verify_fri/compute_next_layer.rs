use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::types::transaction::{EntryFunction, SignedTransaction, TransactionPayload};
use tokio::task::JoinHandle;

use crate::config::AppConfig;
use crate::contracts_caller::transaction_helper::{build_simulated_transaction, build_transaction};
use crate::contracts_caller::verify_fri::types::ComputeNextLayer;
use crate::contracts_caller::vm_status::VmStatus;

pub async fn compute_next_layer(loop_cycles: usize, config: &AppConfig, data: &ComputeNextLayer) -> anyhow::Result<()> {
    let mut txs: Vec<(String, SignedTransaction)> = Vec::with_capacity(loop_cycles);

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
    txs.push(("init_compute_next_layer".to_string(), tx));

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

    for i in 0..loop_cycles {
        let tx = build_transaction(payload.clone(), &config.account, config.chain_id);
        txs.push((format!("compute next layer {}", i).to_string(), tx));
    }

    let pending_transactions = txs.into_iter().map(|(name, transaction)| {
        let client = config.client.clone();
        tokio::spawn(async move {
            let init_transaction = client.submit(&transaction).await.unwrap().into_inner();
            eprintln!("sent {} = {:#?}", name, init_transaction.hash.to_string());
            (name, init_transaction)
        })
    }).collect::<Vec<_>>();

    let mut results = Vec::with_capacity(pending_transactions.len());
    for handle in pending_transactions {
        results.push(handle.await.unwrap());
    }
    let results: Vec<JoinHandle<_>> = results.into_iter().map(|(name, pending_transaction)| {
        let client = config.client.clone();
        tokio::spawn(async move {
            let transaction = client.wait_for_transaction(&pending_transaction).await.unwrap().into_inner();
            let transaction_info = transaction.transaction_info().unwrap();
            eprintln!("finished {} {:#?}; gas used: {}",
                      name,
                      transaction_info.hash.to_string(),
                      transaction_info.gas_used,
            );
            transaction
        })
    }).collect();

    let mut transactions = Vec::new();

    for handle in results {
        let transaction = handle.await.unwrap();
        transactions.push(transaction);
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