use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts_caller::transaction_helper::build_transaction;


pub async fn fmul(config: &AppConfig) -> anyhow::Result<()> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_layer").unwrap()),
            Identifier::new("mul_mod").unwrap(),
            vec![],
            serialize_values(
                &vec![]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let transaction_info = transaction.transaction_info().unwrap();
    println!("mul_mod {}; gas used: {}", transaction_info.hash.to_string(),
             transaction_info.gas_used);
    Ok(())
}



