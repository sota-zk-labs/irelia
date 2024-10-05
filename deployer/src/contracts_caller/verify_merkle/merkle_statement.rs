use std::str::FromStr;

use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::rest_client::aptos_api_types::MoveType;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use log::info;

use crate::config::AppConfig;
use crate::contracts_caller::transaction_helper::{build_transaction, get_event_from_transaction};
use crate::contracts_caller::types::VerifyMerkle;
use crate::contracts_caller::verify_merkle::types::register_fact_verify_merkle::RegisterFactVerifyMerkle;
use crate::contracts_caller::verify_merkle::types::verify_merkle_input::VerifyMerkleTransactionInput;

pub async fn verify_merkle_statement(
    config: &AppConfig,
    data: VerifyMerkleTransactionInput,
) -> anyhow::Result<(VerifyMerkle, RegisterFactVerifyMerkle)> {
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(
            config.module_address,
            Identifier::new("merkle_statement_contract")?,
        ),
        Identifier::new("verify_merkle")?,
        vec![],
        serialize_values(&vec![
            data.merkle_view,
            data.initial_merkle_queue,
            data.height,
            data.expected_root,
        ]),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let transaction_info = transaction.transaction_info()?;
    info!(
        "verify_merkle_statement finished: id={}; hash={}; gas={}",
        transaction_info.version,
        transaction_info.hash.to_string(),
        transaction_info.gas_used
    );

    let verify_merkle_event_type = MoveType::from_str(&format!(
        "{}::merkle_statement_contract::VerifyMerkle",
        config.module_address
    ))?;
    let verify_merkle_data =
        get_event_from_transaction(&transaction, verify_merkle_event_type)?.clone();

    let register_fact_event_type = MoveType::from_str(&format!(
        "{}::merkle_statement_contract::RegisterFactVerifyMerkle",
        config.module_address
    ))?;
    let register_fact_data =
        get_event_from_transaction(&transaction, register_fact_event_type)?.clone();

    let input_verify_merkle: VerifyMerkle = verify_merkle_data.try_into()?;
    let input_register_fact_merkle: RegisterFactVerifyMerkle = register_fact_data.try_into()?;

    Ok((input_verify_merkle, input_register_fact_merkle))
}
