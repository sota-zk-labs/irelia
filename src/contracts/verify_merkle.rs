use anyhow::Error;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts::helper::{build_simulated_transaction, build_transaction, str_to_u256};
use crate::contracts::types::{ComputeNextLayer};
use crate::contracts::vm_status::VmStatus;
use crate::error::CoreError::TransactionNotSucceed;

pub async fn verify_merkle(loop_cycles: usize, config: &AppConfig, data: &ComputeNextLayer, root_hash: &str) -> anyhow::Result<bool> {
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
    let init_transaction = config.client.submit(&tx).await.unwrap().into_inner();
    eprintln!("sent init_verify_merkle = {:#?}", init_transaction.hash.to_string());

    let params = serialize_values(
        &vec![
            MoveValue::U256(data.merkle_queue_ptr.clone()),
            MoveValue::U256(str_to_u256(root_hash)?),
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

    let mut txs = vec![];
    for i in 0..loop_cycles {
        let tx = build_transaction(payload.clone(), &config.account, config.chain_id);
        let transaction = config.client.submit(&tx).await.unwrap().into_inner();
        eprintln!("sent verify merkle {}: {:#?}", i, transaction.hash.to_string());
        txs.push(transaction);
    }


    let init_transaction = config.client.wait_for_transaction(&init_transaction).await.unwrap().into_inner();
    eprintln!("finished init_verify_merkle: {:#?}", init_transaction.transaction_info().unwrap().hash.to_string());

    for i in 0..loop_cycles {
        let transaction = config.client.wait_for_transaction(txs.get(i).unwrap()).await.unwrap().into_inner();
        eprintln!("finished verify merkle {}: {:#?}", i, transaction.transaction_info().unwrap().hash.to_string());
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
