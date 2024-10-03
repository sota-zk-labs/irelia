use std::str::FromStr;

use anyhow::ensure;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::rest_client::aptos_api_types::MoveType;
use aptos_sdk::rest_client::error::RestError;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use log::{debug, info};

use crate::config::AppConfig;
use crate::contracts_caller::gps::types::verify_proof_and_register::VerifyProofAndRegisterData;
use crate::contracts_caller::transaction_helper::{build_transaction, get_event_from_transaction};
use crate::error::CoreError::{FlowNotFinished, TransactionNotSucceed};

pub async fn verify_proof_and_register(
    config: &AppConfig,
    data: &VerifyProofAndRegisterData,
) -> anyhow::Result<()> {
    let module_name = "gps_statement_verifier";
    let mut txs = vec![];

    // Register facts transaction
    // let payload = TransactionPayload::EntryFunction(EntryFunction::new(
    //     ModuleId::new(config.module_address, Identifier::new("fact_registry")?),
    //     Identifier::new("register_facts")?,
    //     vec![],
    //     serialize_values(&vec![MoveValue::Vector(
    //         data.pre_registered_facts
    //             .iter()
    //             .map(|v| MoveValue::U256(*v))
    //             .collect(),
    //     )]),
    // ));
    // let tx = build_transaction(payload, &config.account, config.chain_id);
    // txs.push(("register_facts".to_string(), tx));

    // Prepush task metadata transaction
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(config.module_address, Identifier::new(module_name)?),
        Identifier::new("prepush_task_metadata")?,
        vec![],
        serialize_values(&vec![MoveValue::Vector(
            data.task_metadata
                .iter()
                .map(|v| MoveValue::U256(*v))
                .collect(),
        )]),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    txs.push(("prepush_task_metadata".to_string(), tx));

    // Prepush data to verify proof and register transaction
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(config.module_address, Identifier::new(module_name)?),
        Identifier::new("prepush_data_to_verify_proof_and_register")?,
        vec![],
        serialize_values(&vec![
            MoveValue::Vector(
                data.proof_params
                    .iter()
                    .map(|v| MoveValue::U256(*v))
                    .collect(),
            ),
            MoveValue::Vector(data.proof.iter().map(|v| MoveValue::U256(*v)).collect()),
            MoveValue::Vector(
                data.cairo_aux_input
                    .iter()
                    .map(|v| MoveValue::U256(*v))
                    .collect(),
            ),
            MoveValue::U256(data.cairo_verifier_id),
        ]),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    txs.push(("prepush_data_to_verify_proof_and_register".to_string(), tx));

    // Verify_proof_and_register
    for cnt_loop in 1..=12 {
        debug!("verify_proof_and_register {}", cnt_loop);
        let payload = TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new(module_name)?),
            Identifier::new("verify_proof_and_register")?,
            vec![],
            serialize_values(&vec![]),
        ));
        let tx = build_transaction(payload, &config.account, config.chain_id);
        txs.push((format!("verify_proof_and_register {}", cnt_loop), tx));
    }

    let pending_transactions = txs
        .into_iter()
        .map(|(name, transaction)| {
            let client = config.client.clone();
            tokio::spawn(async move {
                let init_transaction = client.submit(&transaction).await?.into_inner();
                debug!("sent {} hash = {}", name, init_transaction.hash.to_string());
                Ok::<_, RestError>((name, init_transaction))
            })
        })
        .collect::<Vec<_>>();

    let mut results = Vec::with_capacity(pending_transactions.len());
    for handle in pending_transactions {
        results.push(handle.await??);
    }

    let results = results
        .into_iter()
        .map(|(name, pending_transaction)| {
            let client = config.client.clone();
            tokio::spawn(async move {
                let transaction = client
                    .wait_for_transaction(&pending_transaction)
                    .await?
                    .into_inner();
                Ok::<_, anyhow::Error>((name, transaction))
            })
        })
        .collect::<Vec<_>>();

    let mut transactions = Vec::with_capacity(results.len());
    for handle in results {
        let (name, transaction) = handle.await??;
        let transaction_info = transaction.transaction_info()?;
        ensure!(
            transaction_info.success,
            TransactionNotSucceed(format!("{}; hash: {}", name, transaction_info.hash))
        );
        info!(
            "{} finished: id={}; hash={}; gas={}",
            name,
            transaction_info.version,
            transaction_info.hash.to_string(),
            transaction_info.gas_used,
        );
        transactions.push(transaction);
    }

    let last_transaction = transactions.last().unwrap();

    // Get the event from the last transaction
    let event = get_event_from_transaction(
        last_transaction,
        MoveType::from_str(&format!(
            "{}::{}::VparFinished",
            config.module_address, module_name
        ))?,
    );
    ensure!(
        event.is_ok(),
        FlowNotFinished("verify_proof_and_register".to_string())
    );

    Ok(())
}
