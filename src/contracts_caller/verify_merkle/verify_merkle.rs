use aptos_sdk::move_types::value::MoveValue;

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

    if !merkle_verifier(&config, &input_verify_merkle).await? {
        eprintln!("something went wrong!");
        return Ok(());
    }

    if !register_fact_merkle(&config, input_register_fact_merkle).await? {
        eprintln!("something went wrong!");
        return Ok(());
    }

    Ok(())
}