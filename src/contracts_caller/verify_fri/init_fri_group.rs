use aptos_sdk::crypto::HashValue;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use crate::config::AppConfig;
use crate::contracts_caller::helper::{build_transaction};
use crate::contracts_caller::types::InitFriGroup;

pub async fn init_fri_group(config: &AppConfig, data: InitFriGroup) -> anyhow::Result<HashValue> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_layer").unwrap()),
            Identifier::new("init_fri_group").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    data.fri_ctx
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction_info = config.client.submit_and_wait(&tx).await?.into_inner().transaction_info().unwrap().clone();
    let txd = transaction_info.hash;
    println!("init fri group {}; gas used {}", txd.clone(), transaction_info.gas_used);
    Ok(HashValue::from(txd))
}