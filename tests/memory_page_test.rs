#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use aptos_sdk::types::LocalAccount;
    use aptos_testcontainer::test_utils::aptos_container_test_utils::{lazy_aptos_container, run};
    use test_log::test;

    use verifier_onchain_services::config::{AppConfig, EnvConfig};
    use verifier_onchain_services::contracts_caller::memory_page_fact_registry::register_continuous_memory_page::register_continuous_memory_page;
    use verifier_onchain_services::contracts_caller::memory_page_fact_registry::register_continuous_page_batch::register_continuous_page_batch;
    use verifier_onchain_services::contracts_caller::memory_page_fact_registry::sample_register_memory::{sample_large_data_register_continuous_page_batch, sample_register_continuous_page, sample_register_continuous_page_batch};

    #[test(tokio::test)]
    pub async fn memory_page_test() {
        run(2, |accounts| {
            Box::pin(async move {
                let aptos_container = lazy_aptos_container().await.unwrap();
                let node_url = aptos_container.get_node_url();
                let module_account_private_key = accounts.first().unwrap();
                let module_account =
                    LocalAccount::from_private_key(module_account_private_key, 0).unwrap();

                let sender_account_private_key = accounts.get(1).unwrap();

                let module_address = module_account.address().to_string();
                let config = AppConfig::from(EnvConfig {
                    node_url,
                    private_key: sender_account_private_key.to_string(),
                    module_address: module_address.clone(),
                    chain_id: aptos_container.get_chain_id().to_string(),
                });

                let sequence_number = config
                    .client
                    .get_account(config.account.address())
                    .await?
                    .into_inner()
                    .sequence_number;
                config.account.set_sequence_number(sequence_number);

                let mut named_addresses = HashMap::new();
                named_addresses.insert("verifier_addr".to_string(), module_address.clone());
                named_addresses.insert("lib_addr".to_string(), module_address.clone());
                named_addresses.insert("cpu_addr".to_string(), module_address);
                aptos_container
                    .upload_contract(
                        "./contracts/navori",
                        module_account_private_key,
                        &named_addresses,
                        Some(vec!["libs", "cpu", "verifier"]),
                        false,
                    )
                    .await
                    .unwrap();

                let register_continuous_page_batch_input = sample_register_continuous_page_batch()?;
                register_continuous_page_batch(&config, register_continuous_page_batch_input)
                    .await
                    .unwrap();

                let register_continuous_page_input = sample_register_continuous_page()?;
                register_continuous_memory_page(&config, register_continuous_page_input).await?;

                let large_data_register_continuous_page_batch_input =
                    sample_large_data_register_continuous_page_batch()?;
                register_continuous_page_batch(
                    &config,
                    large_data_register_continuous_page_batch_input,
                )
                .await
                .unwrap();
                Ok(())
            })
        })
        .await
        .unwrap()
    }
}
