use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use log::info;

use crate::aptos_writer::config::AppConfig;
use crate::aptos_writer::contracts_caller::transaction_helper::build_transaction;
use crate::aptos_writer::contracts_caller::types::VerifyMerkle;

pub async fn merkle_verifier(config: &AppConfig, data: &VerifyMerkle) -> anyhow::Result<bool> {
    let params = serialize_values(&vec![
        MoveValue::U64(data.channel_ptr),
        MoveValue::U64(data.merkle_queue_ptr),
        MoveValue::U256(data.expected_root),
        MoveValue::U64(data.n_queries),
    ]);
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(config.verifier_address, Identifier::new("merkle_verifier")?),
        Identifier::new("verify_merkle")?,
        vec![],
        params,
    ));

    let tx = build_transaction(payload.clone(), &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let transaction_info = transaction.transaction_info()?;
    info!(
        "verify_merkle finished: id={}; hash={}; gas={}",
        transaction_info.version,
        transaction_info.hash.to_string(),
        transaction_info.gas_used
    );
    Ok(true)
}
