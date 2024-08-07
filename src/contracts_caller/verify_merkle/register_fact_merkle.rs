use std::str::FromStr;
use anyhow::Error;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::rest_client::aptos_api_types::{Event, MoveType};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use crate::config::AppConfig;
use crate::contracts_caller::helper::{build_transaction, get_event_from_transaction};
use crate::contracts_caller::types::RegisterFactVerifyMerkle;
use crate::error::CoreError::TransactionNotSucceed;

pub async fn register_fact_merkle(config: &AppConfig, data: RegisterFactVerifyMerkle) -> anyhow::Result<bool> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("merkle_statement_contract").unwrap()),
            Identifier::new("register_fact_verify_merkle").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    MoveValue::U256( data.channel_ptr),
                    MoveValue::U256(data.data_to_hash_ptr),
                    MoveValue::U256(data.n_queries),
                    MoveValue::U256(data.res_root),
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let vm_status = transaction.success();
    eprintln!("transaction register_fact_verify_merkle = {:#?}", transaction.transaction_info().unwrap().hash.to_string());
    Ok(vm_status)
}