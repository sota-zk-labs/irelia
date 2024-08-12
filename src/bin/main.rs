use verifier_onchain_services::config::{AppConfig, EnvConfig};
use verifier_onchain_services::contracts_caller::verify_fri::sample_verify_fri_input::sample_verify_fri_input;
use verifier_onchain_services::contracts_caller::verify_fri::verify_fri::verify_fri;
use verifier_onchain_services::contracts_caller::verify_merkle::sample_verify_merkle_input::sample_verify_merkle_input;
use verifier_onchain_services::contracts_caller::verify_merkle::verify_merkle::verify_merkle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::from(EnvConfig::new());
    let sequence_number = config.client.get_account(config.account.address()).await?.into_inner().sequence_number;
    config.account.set_sequence_number(sequence_number);

    for i in 1..4 {
        let (merkle_view, initial_merkle_queue, height, expected_root) = sample_verify_merkle_input(i).unwrap();
        verify_merkle(&config, merkle_view, initial_merkle_queue, height, expected_root).await?;
        eprintln!("Verify Merkle {} success", i);
    }

    for i in 1..8 {
        let (fri_verify_input, proof, fri_queue, evaluation_point, fri_step_size, expected_root) = sample_verify_fri_input(i).unwrap();
        verify_fri(&config, fri_verify_input, proof, fri_queue, evaluation_point, fri_step_size, expected_root).await?;
        eprintln!("Verify FRI {} success", i);
    }

    Ok(())
}