use std::str::FromStr;

use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::rest_client::aptos_api_types::{Event, MoveType};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts_caller::transaction_helper::{build_transaction, get_event_from_transaction};
use crate::contracts_caller::verify_fri::types::fri_verify_input::VerifyFriTransactionInput;

pub async fn fri_statement(config: &AppConfig, data: VerifyFriTransactionInput) -> anyhow::Result<(Event, Event, Event)> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_statement_contract").unwrap()),
            Identifier::new("verify_fri").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    data.proof,
                    data.fri_queue,
                    data.evaluation_point,
                    data.fri_step_size,
                    data.expected_root,
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();

    let transaction_info = transaction.transaction_info()?;
    println!("finished fri statement: {}; gas used: {}", transaction_info.hash.to_string(), transaction_info.gas_used);

    let event_type = MoveType::from_str(&format!("{}::fri_statement_contract::FriCtx", config.module_address)).unwrap();
    let fri_ctx_data = get_event_from_transaction(&transaction, event_type)?.clone();

    let compute_next_layer_event_type = MoveType::from_str(&format!("{}::fri_statement_contract::ComputeNextLayer", config.module_address)).unwrap();
    let compute_next_layer_data = get_event_from_transaction(
        &transaction,
        compute_next_layer_event_type,
    )?.clone();

    let register_fact_event_type = MoveType::from_str(&format!("{}::fri_statement_contract::RegisterFactVerifyFri", config.module_address)).unwrap();
    let register_fact_data = get_event_from_transaction(
        &transaction,
        register_fact_event_type,
    )?.clone();

    Ok((fri_ctx_data, compute_next_layer_data, register_fact_data))
}
