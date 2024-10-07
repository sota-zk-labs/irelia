pub mod errors;

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use adapter::proof::Proof;
use aptos_sdk::types::LocalAccount;
use aptos_testcontainer::test_utils::aptos_container_test_utils::{lazy_aptos_container, run};
use deployer::config::{AppConfig, EnvConfig};
use deployer::contracts_caller::gps::extract_gps_input::extract_gps_input;
use deployer::contracts_caller::gps::verify_proof_and_register::verify_proof_and_register;
use deployer::contracts_caller::memory_page_fact_registry::extract_register_memory::extract_register_continuous_page;
use deployer::contracts_caller::memory_page_fact_registry::register_continuous_memory_page::register_continuous_memory_page;
use deployer::contracts_caller::verify_fri::extract_verify_fri_input::extract_verify_fri_input;
use deployer::contracts_caller::verify_fri::types::fri_verify_input;
use deployer::contracts_caller::verify_fri::verify_fri::verify_fri;
use deployer::contracts_caller::verify_merkle::extract_verify_merkle_input::extract_verify_merkle_input;
use deployer::contracts_caller::verify_merkle::verify_merkle::verify_merkle;
use log::info;
use prover::generate_proof;
use stone_cli::args::LayoutName;

use crate::errors::FlowError;
use crate::errors::FlowError::{AdapterError, ProverError, UnsupportedLayoutError};

pub async fn full_flow(
    config: &AppConfig,
    cairo_programs: Option<Vec<PathBuf>>,
    cairo_pies: Option<Vec<PathBuf>>,
    layout: LayoutName,
) -> Result<(), FlowError> {
    if layout != LayoutName::starknet {
        return Err(UnsupportedLayoutError);
    }
    let (topology_json, annotated_proof) = generate_proof(cairo_programs, cairo_pies, layout)
        .map_err(|e| ProverError(e.to_string()))?;

    let proof =
        Proof::new(topology_json, annotated_proof, 6).map_err(|e| AdapterError(e.to_string()))?;

    // verify
    let merkle_verify_inputs = extract_verify_merkle_input(&proof.merkle_proofs).unwrap();
    let fri_verify_inputs = extract_verify_fri_input(&proof.fri_proofs).unwrap();
    let register_continuous_page_inputs =
        extract_register_continuous_page(&proof.memory_pages).unwrap();
    let gps_verify_input = extract_gps_input(&proof.main_proof).unwrap();
    for merkle_verify_input in merkle_verify_inputs {
        let (merkle_view, initial_merkle_queue, height, expected_root) = merkle_verify_input;
        verify_merkle(
            &config,
            merkle_view,
            initial_merkle_queue,
            height,
            expected_root,
        )
        .await
        .unwrap();
        info!("Verify Merkle success");
    }

    for fri_verify_input in fri_verify_inputs {
        let (fri_verify_input, proof, fri_queue, evaluation_point, fri_step_size, expected_root) =
            fri_verify_input;

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
        info!("Verify FRI success",);
    }

    for page in register_continuous_page_inputs {
        register_continuous_memory_page(&config, page)
            .await
            .unwrap();
        info!("Register continuous page success");
    }

    verify_proof_and_register(&config, &gps_verify_input)
        .await
        .unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use aptos_sdk::types::LocalAccount;
    use aptos_testcontainer::test_utils::aptos_container_test_utils::{lazy_aptos_container, run};
    use deployer::config::{AppConfig, EnvConfig};
    use deployer::contracts_caller::gps::verify_proof_and_register::verify_proof_and_register;
    use log::info;
    use stone_cli::args::LayoutName;
    use test_log::test;
    use tokio::time::Instant;

    use crate::full_flow;

    #[test(tokio::test)]
    pub async fn full_flow_test() {
        let cairo_programs = None;
        let cairo_pie = Some(vec![PathBuf::from("./tests/fibonacci_with_output.zip")]);
        let layout = LayoutName::starknet;

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
                named_addresses.insert("cpu_2_addr".to_string(), module_address.clone());
                named_addresses.insert("cpu_addr".to_string(), module_address.clone());
                named_addresses.insert("verifier_addr".to_string(), module_address.clone());

                let now = Instant::now();
                aptos_container
                    .upload_contract(
                        "../contracts/navori",
                        module_account_private_key,
                        &named_addresses,
                        Some(vec!["libs", "cpu-2", "cpu", "verifier"]),
                        false,
                    )
                    .await
                    .unwrap();
                info!("upload_contract: {:.2?}", now.elapsed());

                let now = Instant::now();
                aptos_container
                    .run_script(
                        "../contracts/navori",
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

                full_flow(&config, cairo_programs, cairo_pie, LayoutName::starknet)
                    .await
                    .unwrap();
                Ok(())
            })
        })
        .await
        .unwrap()
    }
}
