#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use aptos_sdk::types::chain_id::NamedChain::TESTING;
    use aptos_sdk::types::LocalAccount;
    use aptos_testcontainer::test_utils::aptos_container_test_utils::{lazy_aptos_container, run};
    use log::info;
    use test_log::test;

    use verifier_onchain_services::config::{AppConfig, EnvConfig};
    use verifier_onchain_services::contracts_caller::verify_fri::sample_verify_fri_input::sample_verify_fri_input;
    use verifier_onchain_services::contracts_caller::verify_merkle::sample_verify_merkle_input::sample_verify_merkle_input;

    #[test(tokio::test)]
    pub async fn verifier_test() {
        run(2, |accounts| {
            Box::pin(async move {
                let aptos_container = lazy_aptos_container().await.unwrap();
                let node_url = aptos_container.get_node_url().await.unwrap();

                let module_account_private_key = accounts.first().unwrap();
                let module_account =
                    LocalAccount::from_private_key(module_account_private_key, 0).unwrap();

                let sender_account_private_key = accounts.get(1).unwrap();
                let sender_account =
                    LocalAccount::from_private_key(sender_account_private_key, 0).unwrap();

                let config = AppConfig::from(EnvConfig {
                    node_url,
                    private_key: sender_account_private_key.to_string(),
                    module_address: module_account.address().to_string(),
                    chain_id: TESTING.id().to_string(),
                });

                let sequence_number = config
                    .client
                    .get_account(config.account.address())
                    .await?
                    .into_inner()
                    .sequence_number;
                config.account.set_sequence_number(sequence_number);

                let mut named_addresses = HashMap::new();
                named_addresses.insert(
                    "verifier_addr".to_string(),
                    module_account.address().to_string(),
                );
                named_addresses
                    .insert("lib_addr".to_string(), module_account.address().to_string());
                aptos_container
                    .upload_contract(
                        "./contracts/navori",
                        module_account_private_key,
                        &named_addresses,
                        Some(vec!["libs", "verifier"]),
                        false,
                    )
                    .await
                    .unwrap();

                for i in 1..4 {
                    sample_verify_merkle_input(&config, i).await.unwrap();
                    info!("Verify Merkle {} success", i);
                }

                for i in 1..8 {
                    sample_verify_fri_input(&config, i).await.unwrap();
                    info!("Verify FRI {} success", i);
                }
                Ok(())
            })
        })
        .await
        .unwrap()
    }
}
