use anyhow::Error;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::types::transaction::{EntryFunction, SignedTransaction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts_caller::transaction_helper::{build_simulated_transaction, build_transaction};
use crate::contracts_caller::types::VerifyMerkle;
use crate::contracts_caller::vm_status::VmStatus;
use crate::error::CoreError::TransactionNotSucceed;

pub async fn merkle_verifier(loop_cycles: usize, config: &AppConfig, data: &VerifyMerkle) -> anyhow::Result<bool> {
    let mut txs: Vec<(String, SignedTransaction)> = Vec::with_capacity(loop_cycles + 1);

    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("merkle_verifier").unwrap()),
            Identifier::new("init_verify_merkle").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    MoveValue::U256(data.channel_ptr.clone()),
                    MoveValue::U256(data.merkle_queue_ptr.clone()),
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    txs.push(("init_verify_merkle".to_string(), tx));

    let params = serialize_values(
        &vec![
            MoveValue::U256(data.channel_ptr.clone()),
            MoveValue::U256(data.merkle_queue_ptr.clone()),
            MoveValue::U256(data.expected_root.clone()),
            MoveValue::U256(data.n_queries.clone()),
        ]
    );
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("merkle_verifier").unwrap()),
            Identifier::new("verify_merkle").unwrap(),
            vec![],
            params,
        ));

    for i in 0..loop_cycles {
        let tx = build_transaction(payload.clone(), &config.account, config.chain_id);
        txs.push((format!("sent verify merkle {}", i).to_string(), tx));
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

    let results = results.into_iter().map(|(name, pending_transaction)| {
        let client = config.client.clone();
        tokio::spawn(async move {
            let transaction = client.wait_for_transaction(&pending_transaction).await.unwrap().into_inner();
            let transaction_info = transaction.transaction_info().unwrap();
            eprintln!("finished {} {:#?}; gas used: {}",
                      name,
                      transaction_info.hash.to_string(),
                      transaction_info.gas_used,
            );
        })
    }).collect::<Vec<_>>();

    for handle in results {
        handle.await.unwrap();
    }

    let tx = build_simulated_transaction(payload.clone(), &config.account, config.chain_id);
    let simulate = config.client.simulate(&tx).await.unwrap().into_inner();
    if simulate.get(0).unwrap().info.success {
        return Ok(false);
    }

    let vm_status = simulate.get(0).unwrap().info.vm_status.as_str();
    let vm_status: VmStatus = vm_status.try_into()?;
    if vm_status.reason == "EVERIFY_MERKLE_NOT_INITIATED" {
        println!("verify_merkle check passes");
        return Ok(true);
    }

    return Err(Error::new(TransactionNotSucceed(format!("{}", vm_status).to_string())));
}
