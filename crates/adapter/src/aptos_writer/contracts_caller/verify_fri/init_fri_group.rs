use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use log::info;

use crate::aptos_writer::config::AppConfig;
use crate::aptos_writer::contracts_caller::transaction_helper::build_transaction;
use crate::aptos_writer::contracts_caller::verify_fri::types::init_fri_group::InitFriGroup;

pub async fn init_fri_group(config: &AppConfig, data: InitFriGroup) -> anyhow::Result<bool> {
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(config.module_address, Identifier::new("fri_layer")?),
        Identifier::new("init_fri_group")?,
        vec![],
        serialize_values(&vec![MoveValue::U64(data.fri_ctx)]),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction_info = config
        .client
        .submit_and_wait(&tx)
        .await?
        .into_inner()
        .transaction_info()?
        .clone();
    info!(
        "init_fri_group finished: id={}; hash={}; gas={}",
        transaction_info.version,
        transaction_info.hash.to_string(),
        transaction_info.gas_used
    );
    Ok(transaction_info.success)
}
