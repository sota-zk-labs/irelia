use std::str::FromStr;

use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use itertools::Itertools;
use log::info;

use crate::config::AppConfig;
use crate::contracts_caller::memory_page_fact_registry::types::register_continuous_page_batch::MemoryPageEntries;
use crate::contracts_caller::transaction_helper::build_transaction;

pub async fn register_continuous_page_batch(
    config: &AppConfig,
    data: MemoryPageEntries,
) -> anyhow::Result<bool> {
    let data_input = data.memory_page_entries;
    let initial_chunk_size = 15;

    let mut start_addr_values = vec![];

    for e in &data_input {
        let start_addr = MoveValue::U256(U256::from_str(&e.start_addr)?);
        let mut value = vec![];
        for v in &e.values {
            value.push(MoveValue::U256(U256::from_str(v)?));
        }
        start_addr_values.push((start_addr, MoveValue::Vector(value)));
    }

    start_addr_values.sort_by_key(|(_, values)| match values {
        MoveValue::Vector(v) => v.len(),
        _ => 0,
    });

    let mut chunk_size = initial_chunk_size;
    let mut success = true;
    let mut remaining_data = start_addr_values;

    while chunk_size > 0 {
        success = true;
        let mut new_remaining_data = vec![];

        for chunk in &remaining_data.iter().chunks(chunk_size) {
            let chunk: Vec<_> = chunk.cloned().collect();
            let mut chunk_start_addr = vec![];
            let mut chunk_values = vec![];

            for (addr, val) in &chunk {
                chunk_start_addr.push(addr.clone());
                chunk_values.push(val.clone());
            }

            let payload = TransactionPayload::EntryFunction(EntryFunction::new(
                ModuleId::new(
                    config.module_address,
                    Identifier::new("memory_page_fact_registry")?,
                ),
                Identifier::new("register_continuous_page_batch")?,
                vec![],
                serialize_values(&vec![
                    MoveValue::Vector(chunk_start_addr),
                    MoveValue::Vector(chunk_values),
                    MoveValue::U256(U256::from_str(&data_input.first().unwrap().z)?),
                    MoveValue::U256(U256::from_str(&data_input.first().unwrap().alpha)?),
                    MoveValue::U256(U256::from_str(&data_input.first().unwrap().prime)?),
                ]),
            ));
            let tx = build_transaction(payload, &config.account, config.chain_id);
            let transaction = match config.client.submit_and_wait(&tx).await {
                Ok(tx) => tx.into_inner(),
                Err(_) => {
                    success = false;
                    new_remaining_data.extend(chunk);
                    break;
                }
            };
            let transaction_info = transaction.transaction_info()?;
            info!(
                "register_continuous_memory_page_batch: {}; gas used: {}",
                transaction_info.hash.to_string(),
                transaction_info.gas_used
            );
        }

        remaining_data = new_remaining_data;
        chunk_size /= 2;
    }
    Ok(success)
}
