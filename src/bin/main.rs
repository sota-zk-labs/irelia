use log::{error, info};

use verifier_onchain_services::config::{AppConfig, EnvConfig};
use verifier_onchain_services::contracts_caller::memory_page_fact_registry::sample_register_memory::sample_register_continuous_page_batch;
use verifier_onchain_services::contracts_caller::verify_fri::sample_verify_fri_input::sample_verify_fri_input;
use verifier_onchain_services::contracts_caller::verify_merkle::sample_verify_merkle_input::sample_verify_merkle_input;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::from(EnvConfig::new());
    let sequence_number = config
        .client
        .get_account(config.account.address())
        .await?
        .into_inner()
        .sequence_number;
    config.account.set_sequence_number(sequence_number);

    for i in 1..4 {
        sample_verify_merkle_input(&config, i).await?;
        info!("Verify Merkle {} success", i);
    }

    for i in 1..8 {
        sample_verify_fri_input(&config, i).await?;
        info!("Verify FRI {} success", i);
    }

    if !sample_register_continuous_page_batch(&config).await? {
        error!("something went wrong!")
    }

    Ok(())
}
