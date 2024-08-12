use std::str::FromStr;

use aptos_sdk::move_types::value::MoveValue;
use aptos_sdk::rest_client::aptos_api_types::{EntryFunctionId, ViewRequest};

use crate::config::AppConfig;
use crate::contracts_caller::types::VerifyMerkle;
use crate::contracts_caller::verify_fri::merkle_verifier::merkle_verifier;
use crate::contracts_caller::verify_merkle::merkle_statement::verify_merkle_statement;
use crate::contracts_caller::verify_merkle::register_fact_merkle::register_fact_merkle;
use crate::contracts_caller::verify_merkle::types::register_fact_verify_merkle::RegisterFactVerifyMerkle;
use crate::contracts_caller::verify_merkle::types::verify_merkle_input::VerifyMerkleTransactionInput;

pub async fn verify_merkle(
    config: &AppConfig,
    merkle_view: MoveValue,
    initial_merkle_queue: MoveValue,
    height: MoveValue,
    expected_root: MoveValue,
) -> anyhow::Result<()> {
    let verify_merkle_input = VerifyMerkleTransactionInput {
        merkle_view,
        initial_merkle_queue,
        height,
        expected_root,
    };

    let (event_verify_merkle, event_register_fact_merkle) = verify_merkle_statement(&config, verify_merkle_input).await?;

    let input_verify_merkle: VerifyMerkle = event_verify_merkle.try_into()?;
    let input_register_fact_merkle: RegisterFactVerifyMerkle = event_register_fact_merkle.try_into()?;

    let count_verify_merkle_cycles_request = config.client.view(&ViewRequest {
        function: EntryFunctionId::from_str(format!("{}::merkle_verifier::count_verify_merkle_cycles", config.module_address).as_str()).unwrap(),
        type_arguments: vec![],
        arguments: vec![
            serde_json::Value::String(config.account.address().to_string()),
            serde_json::Value::String(input_verify_merkle.merkle_queue_ptr.to_string()),
            serde_json::Value::String(input_verify_merkle.n_queries.to_string()),
        ],
    }, None).await?;
    let verify_merkle_cycles = count_verify_merkle_cycles_request.into_inner().remove(0).as_str().unwrap().parse::<usize>().unwrap();
    eprintln!("verify_merkle_cycles = {:#?}", verify_merkle_cycles);

    if !merkle_verifier(verify_merkle_cycles, &config, &input_verify_merkle).await? {
        eprintln!("something went wrong!");
        return Ok(());
    }

    if !register_fact_merkle(&config, input_register_fact_merkle).await? {
        eprintln!("something went wrong!");
        return Ok(());
    }

    Ok(())
}