use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use log::info;

use crate::config::AppConfig;
use crate::contracts_caller::transaction_helper::build_transaction;
use crate::contracts_caller::verify_fri::types::register_fact_verify_fri::RegisterFactVerifyFri;

pub async fn register_fact_fri(
    config: &AppConfig,
    data: RegisterFactVerifyFri,
    n_queries: u64,
) -> anyhow::Result<bool> {
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(
            config.module_address,
            Identifier::new("fri_statement_contract")?,
        ),
        Identifier::new("register_fact_verify_fri")?,
        vec![],
        serialize_values(&vec![
            MoveValue::U64(data.data_to_hash),
            MoveValue::U64(data.fri_queue_ptr),
            MoveValue::U64(n_queries),
        ]),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let transaction_status = transaction.success();
    let transaction_info = transaction.transaction_info()?;
    info!(
        "register_fact_verify_fri finished: id={}; hash={}; gas={}",
        transaction_info.version,
        transaction_info.hash.to_string(),
        transaction_info.gas_used
    );
    Ok(transaction_status)
}
