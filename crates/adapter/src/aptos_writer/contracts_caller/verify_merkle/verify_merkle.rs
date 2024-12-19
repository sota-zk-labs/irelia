use irelia_core::entities::merkle_statement::VerifyMerkleTransactionInput;
use log::error;

use crate::aptos_writer::config::AppConfig;
use crate::aptos_writer::contracts_caller::verify_fri::merkle_verifier::merkle_verifier;
use crate::aptos_writer::contracts_caller::verify_merkle::merkle_statement::verify_merkle_statement;
use crate::aptos_writer::contracts_caller::verify_merkle::register_fact_merkle::register_fact_merkle;

pub async fn verify_merkle(
    config: &AppConfig,
    verify_merkle_input: VerifyMerkleTransactionInput,
) -> anyhow::Result<()> {
    let (input_verify_merkle, input_register_fact_merkle) =
        verify_merkle_statement(config, verify_merkle_input).await?;

    if !merkle_verifier(config, &input_verify_merkle).await? {
        error!("something went wrong!");
        return Ok(());
    }

    if !register_fact_merkle(config, &input_register_fact_merkle).await? {
        error!("something went wrong!");
        return Ok(());
    }

    Ok(())
}
