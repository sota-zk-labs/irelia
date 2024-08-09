use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts_caller::transaction_helper::build_transaction;
use crate::contracts_caller::verify_fri::types::RegisterFactVerifyFri;

pub async fn register_fact_fri(config: &AppConfig, data: RegisterFactVerifyFri, n_queries: U256) -> anyhow::Result<bool> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_statement_contract").unwrap()),
            Identifier::new("register_fact_verify_fri").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    MoveValue::U256(data.data_to_hash),
                    MoveValue::U256(data.fri_queue_ptr),
                    MoveValue::U256(n_queries),
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let vm_status = transaction.success();
    eprintln!("transaction register_fact_verify_fri = {:#?}", transaction.transaction_info().unwrap().hash.to_string());
    Ok(vm_status)
}