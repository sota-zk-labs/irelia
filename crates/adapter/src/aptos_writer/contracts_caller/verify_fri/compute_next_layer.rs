use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use log::info;

use crate::aptos_writer::config::AppConfig;
use crate::aptos_writer::contracts_caller::transaction_helper::build_transaction;
use crate::aptos_writer::contracts_caller::verify_fri::types::compute_next_layer::ComputeNextLayer;

pub async fn compute_next_layer(
    config: &AppConfig,
    data: &ComputeNextLayer,
) -> anyhow::Result<bool> {
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(config.verifier_address, Identifier::new("fri_layer")?),
        Identifier::new("compute_next_layer")?,
        vec![],
        serialize_values(&vec![
            MoveValue::U64(data.channel_ptr),
            MoveValue::U64(data.fri_queue_ptr),
            MoveValue::U64(data.merkle_queue_ptr),
            MoveValue::U64(data.n_queries),
            MoveValue::U64(data.fri_ctx),
            MoveValue::U256(data.evaluation_point),
            MoveValue::U64(data.fri_coset_size),
        ]),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let transaction_info = transaction.transaction_info()?;
    info!(
        "compute_next_layer finished: id={}; hash={}; gas={}",
        transaction_info.version,
        transaction_info.hash.to_string(),
        transaction_info.gas_used
    );
    Ok(transaction_info.success)
}
