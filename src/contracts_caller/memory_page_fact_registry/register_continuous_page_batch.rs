use crate::config::AppConfig;
use crate::contracts_caller::memory_page_fact_registry::types::memory_page_batch::MemoryPageEntries;
use crate::contracts_caller::transaction_helper::{build_transaction, get_events_from_transaction};
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::rest_client::aptos_api_types::MoveType;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use std::str::FromStr;

pub async fn register_continuous_page_batch(
    config: &AppConfig,
    data: MemoryPageEntries,
) -> anyhow::Result<bool> {
    let data_input = data.memory_page_entries;

    let mut start_addr = vec![];
    let mut values = vec![];
    for e in &data_input {
        start_addr.push(MoveValue::U256(U256::from_str(&e.start_addr)?));
        let mut value = vec![];
        for v in &e.values {
            value.push(MoveValue::U256(U256::from_str(v)?));
        }
        values.push(MoveValue::Vector(value));
    }

    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(
            config.module_address,
            Identifier::new("memory_page_fact_registry")?,
        ),
        Identifier::new("register_continuous_page_batch")?,
        vec![],
        serialize_values(&vec![
            MoveValue::Vector(start_addr),
            MoveValue::Vector(values),
            MoveValue::U256(U256::from_str(&data_input.get(0).unwrap().z)?),
            MoveValue::U256(U256::from_str(&data_input.get(0).unwrap().alpha)?),
            MoveValue::U256(U256::from_str(&data_input.get(0).unwrap().prime)?),
        ]),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();

    let log_memory_page_fact_continuous = MoveType::from_str(&format!(
        "{}::memory_page_fact_registry::LogMemoryPageFactContinuous",
        config.module_address
    ))?;
    let event = get_events_from_transaction(&transaction, log_memory_page_fact_continuous)?;
    Ok(transaction.success())
}
