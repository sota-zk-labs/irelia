use std::str::FromStr;

use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::rest_client::aptos_api_types::MoveType;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts_caller::transaction_helper::{build_transaction, get_event_from_transaction};
use crate::contracts_caller::verify_fri::types::compute_next_layer::ComputeNextLayer;
use crate::contracts_caller::verify_fri::types::fri_verify_input::VerifyFriTransactionInput;
use crate::contracts_caller::verify_fri::types::init_fri_group::InitFriGroup;
use crate::contracts_caller::verify_fri::types::register_fact_verify_fri::RegisterFactVerifyFri;

pub async fn fri_statement(
    config: &AppConfig,
    data: VerifyFriTransactionInput,
) -> anyhow::Result<(InitFriGroup, ComputeNextLayer, RegisterFactVerifyFri)> {
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(
            config.module_address,
            Identifier::new("fri_statement_contract")?,
        ),
        Identifier::new("verify_fri")?,
        vec![],
        serialize_values(&vec![
            data.proof,
            data.fri_queue,
            data.evaluation_point,
            data.fri_step_size,
            data.expected_root,
        ]),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();

    let event_type = MoveType::from_str(&format!(
        "{}::fri_statement_contract::FriCtx",
        config.module_address
    ))?;
    let fri_ctx_data: InitFriGroup = get_event_from_transaction(&transaction, event_type)?
        .clone()
        .try_into()?;

    let compute_next_layer_event_type = MoveType::from_str(&format!(
        "{}::fri_statement_contract::ComputeNextLayer",
        config.module_address
    ))?;
    let compute_next_layer_data: ComputeNextLayer =
        get_event_from_transaction(&transaction, compute_next_layer_event_type)?
            .clone()
            .try_into()?;

    let register_fact_event_type = MoveType::from_str(&format!(
        "{}::fri_statement_contract::RegisterFactVerifyFri",
        config.module_address
    ))?;
    let register_fact_data: RegisterFactVerifyFri =
        get_event_from_transaction(&transaction, register_fact_event_type)?
            .clone()
            .try_into()?;

    Ok((fri_ctx_data, compute_next_layer_data, register_fact_data))
}
