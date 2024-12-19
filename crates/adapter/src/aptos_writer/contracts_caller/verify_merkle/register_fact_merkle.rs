use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use log::info;

use crate::aptos_writer::config::AppConfig;
use crate::aptos_writer::contracts_caller::transaction_helper::build_transaction;
use crate::aptos_writer::contracts_caller::verify_merkle::types::register_fact_verify_merkle::RegisterFactVerifyMerkle;

pub async fn register_fact_merkle(
    config: &AppConfig,
    data: &RegisterFactVerifyMerkle,
) -> anyhow::Result<bool> {
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(
            config.verifier_address,
            Identifier::new("merkle_statement_contract")?,
        ),
        Identifier::new("register_fact_verify_merkle")?,
        vec![],
        serialize_values(&vec![
            MoveValue::U64(data.channel_ptr),
            MoveValue::U64(data.data_to_hash_ptr),
            MoveValue::U64(data.n_queries),
            MoveValue::U256(data.res_root),
        ]),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let vm_status = transaction.success();
    info!(
        "transaction register_fact_verify_merkle = {:#?}",
        transaction.transaction_info()?.hash.to_string()
    );
    Ok(vm_status)
}
