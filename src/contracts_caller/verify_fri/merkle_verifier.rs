use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts_caller::transaction_helper::build_transaction;
use crate::contracts_caller::types::VerifyMerkle;

pub async fn merkle_verifier(config: &AppConfig, data: &VerifyMerkle) -> anyhow::Result<bool> {

    let params = serialize_values(
        &vec![
            MoveValue::U256(data.channel_ptr.clone()),
            MoveValue::U256(data.merkle_queue_ptr.clone()),
            MoveValue::U256(data.expected_root.clone()),
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


    let tx = build_transaction(payload.clone(), &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let transaction_info = transaction.transaction_info().unwrap();
    eprintln!("finished verify_merkle {}; gas used: {}", transaction_info.hash.to_string(),
              transaction_info.gas_used);
    Ok(transaction_info.success)
}
