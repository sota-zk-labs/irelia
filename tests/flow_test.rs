#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;

    use aptos_sdk::types::LocalAccount;
    use aptos_testcontainer::test_utils::aptos_container_test_utils::{lazy_aptos_container, run};
    use log::info;
    use test_log::test;
    use tokio::time::Instant;
    use verifier_onchain_services::config::{AppConfig, EnvConfig};
    use verifier_onchain_services::contracts_caller::gps::types::verify_proof_and_register::{
        VerifyProofAndRegisterData, VerifyProofAndRegisterDataJson,
    };
    use verifier_onchain_services::contracts_caller::gps::verify_proof_and_register::verify_proof_and_register;
    use verifier_onchain_services::contracts_caller::verify_fri::sample_verify_fri_input::sample_verify_fri_input;
    use verifier_onchain_services::contracts_caller::verify_fri::verify_fri::verify_fri;
    use verifier_onchain_services::contracts_caller::verify_merkle::sample_verify_merkle_input::sample_verify_merkle_input;
    use verifier_onchain_services::contracts_caller::verify_merkle::verify_merkle::verify_merkle;

    #[test(tokio::test)]
    pub async fn verifier_test() {
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

                let mut named_addresses = HashMap::new();
                named_addresses.insert("lib_addr".to_string(), module_address.clone());
                named_addresses.insert("cpu_addr".to_string(), module_address.clone());
                named_addresses.insert("verifier_addr".to_string(), module_address.clone());
                named_addresses.insert(
                    "cpu_constraint_poly_addr".to_string(),
                    module_address.clone(),
                );

                let now = Instant::now();
                aptos_container
                    .upload_contract(
                        "./contracts/navori",
                        module_account_private_key,
                        &named_addresses,
                        Some(vec!["libs", "cpu", "cpu-constraint-poly", "verifier"]),
                        false,
                    )
                    .await
                    .unwrap();
                info!("upload_contract: {:.2?}", now.elapsed());

                let now = Instant::now();
                aptos_container
                    .run_script(
                        "./contracts/navori",
                        sender_account_private_key,
                        &named_addresses,
                        &vec!["verifier"],
                    )
                    .await
                    .unwrap();
                info!("run_script: {:.2?}", now.elapsed());

                let sequence_number = config
                    .client
                    .get_account(config.account.address())
                    .await?
                    .into_inner()
                    .sequence_number;
                config.account.set_sequence_number(sequence_number);

                for i in 1..4 {
                    let (merkle_view, initial_merkle_queue, height, expected_root) =
                        sample_verify_merkle_input(i).unwrap();
                    verify_merkle(
                        &config,
                        merkle_view,
                        initial_merkle_queue,
                        height,
                        expected_root,
                    )
                    .await
                    .unwrap();
                    info!("Verify Merkle {} success", i);
                }

                for i in 1..8 {
                    let (
                        fri_verify_input,
                        proof,
                        fri_queue,
                        evaluation_point,
                        fri_step_size,
                        expected_root,
                    ) = sample_verify_fri_input(i).unwrap();
                    verify_fri(
                        &config,
                        fri_verify_input,
                        proof,
                        fri_queue,
                        evaluation_point,
                        fri_step_size,
                        expected_root,
                    )
                    .await
                    .unwrap();
                    info!("Verify FRI {} success", i);
                }
                Ok(())
            })
        })
        .await
        .unwrap()
    }

    #[test(tokio::test)]
    pub async fn verify_proof_and_register_test() {
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

                let mut named_addresses = HashMap::new();
                named_addresses
                    .insert("lib_addr".to_string(), module_account.address().to_string());
                named_addresses
                    .insert("cpu_addr".to_string(), module_account.address().to_string());
                named_addresses.insert(
                    "verifier_addr".to_string(),
                    module_account.address().to_string(),
                );
                named_addresses.insert(
                    "cpu_constraint_poly_addr".to_string(),
                    module_account.address().to_string(),
                );

                let now = Instant::now();
                aptos_container
                    .upload_contract(
                        "./contracts/navori",
                        module_account_private_key,
                        &named_addresses,
                        Some(vec!["libs", "cpu", "cpu-constraint-poly", "verifier"]),
                        false,
                    )
                    .await
                    .unwrap();
                info!("upload_contract: {:.2?}", now.elapsed());

                let now = Instant::now();
                aptos_container
                    .run_script(
                        "./contracts/navori",
                        sender_account_private_key,
                        &named_addresses,
                        &vec!["verifier"],
                    )
                    .await
                    .unwrap();
                info!("run_script: {:.2?}", now.elapsed());

                let sequence_number = config
                    .client
                    .get_account(config.account.address())
                    .await?
                    .into_inner()
                    .sequence_number;
                config.account.set_sequence_number(sequence_number);

                verify_proof_and_register(&config, &sample_vpar_data(1).unwrap())
                    .await
                    .unwrap();
                Ok(())
            })
        })
        .await
        .unwrap()
    }

    fn sample_vpar_data(test_num: isize) -> anyhow::Result<VerifyProofAndRegisterData> {
        let data = serde_json::from_str::<VerifyProofAndRegisterDataJson>(
            fs::read_to_string(format!(
                "src/data_samples/gps/verify_proof_and_register_{}.json",
                test_num
            ))?
            .as_str(),
        )?;
        Ok(VerifyProofAndRegisterData::from(data))
    }
}
