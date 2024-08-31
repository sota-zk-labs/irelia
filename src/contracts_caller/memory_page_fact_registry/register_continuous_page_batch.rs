use std::str::FromStr;

use anyhow::ensure;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::rest_client::error::RestError;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use log::{debug, info};

use crate::config::AppConfig;
use crate::contracts_caller::memory_page_fact_registry::types::register_continuous_memory_page::ContinuousMemoryPage;
use crate::contracts_caller::memory_page_fact_registry::types::register_continuous_page_batch::MemoryPageEntries;
use crate::contracts_caller::transaction_helper::build_transaction;
use crate::error::CoreError::TransactionNotSucceed;

const MAX_MEMORY_VALUE_LEN: usize = 500;

pub async fn register_continuous_page_batch(
    config: &AppConfig,
    data: MemoryPageEntries,
) -> anyhow::Result<()> {
    let ContinuousMemoryPage {
        z,
        alpha,
        prime,
        values: _,
        start_addr: _,
    } = data.memory_page_entries.first().unwrap();

    let z = MoveValue::U256(U256::from_str(z)?);
    let alpha = MoveValue::U256(U256::from_str(alpha)?);
    let prime = MoveValue::U256(U256::from_str(prime)?);

    let mut converted_data = data
        .memory_page_entries
        .into_iter()
        .map(|entry| {
            let start_addr = MoveValue::U256(U256::from_str(&entry.start_addr).unwrap());
            let values = entry
                .values
                .into_iter()
                .map(|value| MoveValue::U256(U256::from_str(&value).unwrap()))
                .collect::<Vec<_>>();
            (start_addr, values.len(), MoveValue::Vector(values))
        })
        .collect::<Vec<_>>();
    converted_data.sort_by_key(|(_, _, values)| match values {
        MoveValue::Vector(v) => v.len(),
        _ => 0,
    });

    let (mut chunks, cur_el, _) = converted_data.into_iter().fold(
        (vec![], vec![], 0),
        |(mut chunks, mut cur_el, mut cur_value_len), (start_addr, values_len, values)| {
            let new_value_len = cur_value_len + values_len;
            if cur_el.len() == 0 {
                cur_el.push((start_addr, values));
                cur_value_len += values_len;
                return (chunks, cur_el, cur_value_len);
            }

            if new_value_len > MAX_MEMORY_VALUE_LEN {
                chunks.push(cur_el);
                cur_value_len = values_len;
                cur_el = vec![(start_addr, values)];
                return (chunks, cur_el, cur_value_len);
            }

            cur_value_len = new_value_len;
            cur_el.push((start_addr, values));
            (chunks, cur_el, cur_value_len)
        },
    );

    if cur_el.len() != 0 {
        chunks.push(cur_el);
    }

    let txs = chunks.into_iter().enumerate().map(|(i, chunk)| {
        let (chunk_start_addr, chunk_values): (Vec<_>, Vec<_>) = chunk.into_iter().unzip();

        let payload = TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(
                config.module_address,
                Identifier::new("memory_page_fact_registry").unwrap(),
            ),
            Identifier::new("register_continuous_page_batch").unwrap(),
            vec![],
            serialize_values(&vec![
                MoveValue::Vector(chunk_start_addr),
                MoveValue::Vector(chunk_values),
                z.clone(),
                alpha.clone(),
                prime.clone(),
            ]),
        ));
        let tx = build_transaction(payload, &config.account, config.chain_id);
        (format!("register_continuous_memory_page_batch_{}", i), tx)
    });

    let pending_transactions = txs
        .into_iter()
        .map(|(name, transaction)| {
            let client = config.client.clone();
            tokio::spawn(async move {
                let init_transaction = client.submit(&transaction).await?.into_inner();
                debug!(
                    "sent {}: hash = {}",
                    name,
                    init_transaction.hash.to_string()
                );
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
                Ok(())
            })
        })
        .collect::<Vec<_>>();

    for handle in results {
        handle.await??;
    }

    Ok(())
}
