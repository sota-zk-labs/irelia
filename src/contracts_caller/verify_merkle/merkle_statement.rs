use std::str::FromStr;

use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::rest_client::aptos_api_types::{Event, MoveType};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts_caller::helper::{build_transaction, get_event_from_transaction};
use crate::contracts_caller::types::VerifyMerkleTransactionInput;

pub async fn verify_merkle_statement(config: &AppConfig, data: VerifyMerkleTransactionInput) -> anyhow::Result<(Event, Event)> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("merkle_statement_contract").unwrap()),
            Identifier::new("verify_merkle").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    data.merkle_view,
                    data.initial_merkle_queue,
                    data.height,
                    data.expected_root,
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let verify_merkle_event_type = MoveType::from_str(&format!("{}::merkle_statement_contract::VerifyMerkle", config.module_address)).unwrap();
    let verify_merkle_data = get_event_from_transaction(
        transaction.clone(),
        verify_merkle_event_type,
    ).await?;

    let register_fact_event_type = MoveType::from_str(&format!("{}::merkle_statement_contract::RegisterFactVerifyMerkle", config.module_address)).unwrap();
    let register_fact_data = get_event_from_transaction(
        transaction,
        register_fact_event_type,
    ).await?;

    Ok((verify_merkle_data, register_fact_data))
}