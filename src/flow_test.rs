#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use aptos_sdk::crypto::ValidCryptoMaterialStringExt;
    use aptos_sdk::types::chain_id::NamedChain::TESTING;
    use aptos_sdk::types::LocalAccount;
    use aptos_testcontainer::test_utils::aptos_container_test_utils::{lazy_aptos_container, run};
    use tokio::time::Instant;

    use crate::config::AppConfig;
    use crate::config::EnvConfig;
    use crate::contracts_caller::verify_fri::sample_verify_fri_input::sample_verify_fri_input;
    use crate::contracts_caller::verify_fri::verify_fri::verify_fri;
    use crate::contracts_caller::verify_merkle::sample_verify_merkle_input::sample_verify_merkle_input;
    use crate::contracts_caller::verify_merkle::verify_merkle::verify_merkle;

    #[tokio::test]
    pub async fn verifier_test() -> anyhow::Result<()> {
        let aptos_container = lazy_aptos_container().await.unwrap();
        run(2, |accounts| {
            Box::pin(async move {
                let aptos_container = lazy_aptos_container().await.unwrap();
                let node_url = aptos_container.get_node_url().await.unwrap();

                let module_account_private_key = accounts.get(0).unwrap();
                let module_account = LocalAccount::from_private_key(module_account_private_key, 0).unwrap();

                let sender_account_private_key = accounts.get(1).unwrap();
                let sender_account = LocalAccount::from_private_key(sender_account_private_key, 0).unwrap();

                let config = AppConfig::from(EnvConfig {
                    node_url,
                    private_key: sender_account_private_key.to_string(),
                    module_address: module_account.address().to_string(),
                    chain_id: TESTING.id().to_string(),
                });

                let sequence_number = config.client.get_account(config.account.address()).await?.into_inner().sequence_number;
                config.account.set_sequence_number(sequence_number);

                let mut named_addresses = HashMap::new();
                named_addresses.insert("verifier_addr".to_string(), module_account.address().to_string());
                aptos_container.upload_contract("contract-sample/navori", &module_account.private_key().to_encoded_string().unwrap(), &named_addresses, false).await.unwrap();

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
            })
        }).await
    }
}