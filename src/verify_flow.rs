#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::str::FromStr;

    use aptos_sdk::move_types::u256::U256;
    use aptos_sdk::move_types::value::MoveValue;
    use aptos_sdk::rest_client::aptos_api_types::{EntryFunctionId, ViewRequest};
    use aptos_sdk::types::LocalAccount;
    use aptos_testcontainer::test_config::aptos_container_test::lazy_aptos_container;

    use crate::config::AppConfig;
    use crate::config::EnvConfig;
    use crate::contracts_caller::types::VerifyMerkle;
    use crate::contracts_caller::verify_fri::compute_next_layer::{compute_next_layer, simulate_compute_next_layer};
    use crate::contracts_caller::verify_fri::init_fri_group::init_fri_group;
    use crate::contracts_caller::verify_fri::register_fact_fri::register_fact_fri;
    use crate::contracts_caller::verify_fri::types::{ComputeNextLayer, FriVerifyInput, InitFriGroup, RegisterFactVerifyFri, VerifyFriTransactionInput};
    use crate::contracts_caller::verify_fri::verify_fri::verify_fri;
    use crate::contracts_caller::verify_fri::verify_merkle::verify_merkle;
    use crate::contracts_caller::verify_merkle::merkle_statement::verify_merkle_statement;
    use crate::contracts_caller::verify_merkle::register_fact_merkle::register_fact_merkle;
    use crate::contracts_caller::verify_merkle::types::{MerkleVerifyInput, RegisterFactVerifyMerkle, VerifyMerkleTransactionInput};

    #[tokio::test]
    pub async fn test_a() -> anyhow::Result<()> {
        let aptos_container = lazy_aptos_container().await.unwrap();

        let node_url = aptos_container.get_node_url().await.unwrap();
        let sender_account = LocalAccount::from_private_key("0xa7599766d8aaace6959eb7e315c1c76af44276641dff8912c9356e3d0799c80d", 0).unwrap();
        let module_account = LocalAccount::from_private_key("0x73791ce34b2414d4afcb87561b0c442e48a3260f1c96de31da80f7cf2eec8113", 0).unwrap();

        aptos_container.faucet(&module_account).await.unwrap();
        aptos_container.faucet(&sender_account).await.unwrap();

        let config = AppConfig::from(EnvConfig {
            node_url,
            private_key: "0xa7599766d8aaace6959eb7e315c1c76af44276641dff8912c9356e3d0799c80d".to_string(),
            module_address: module_account.address().to_string(),
            chain_id: "4".to_string(),
        });

        let sequence_number = config.client.get_account(config.account.address()).await?.into_inner().sequence_number;
        config.account.set_sequence_number(sequence_number);

        let mut named_addresses = HashMap::new();
        named_addresses.insert("verifier_addr".to_string(), module_account.address().to_string());
        aptos_container.upload_contract("./contract-sample/navori", &module_account, &named_addresses, true).await.unwrap();


        for i in 1..4 {
            let file_path = format!("./src/data_samples/merkle_verify/merkle_verify_{}.json", i);
            let input_file = File::open(file_path)?;
            let reader = BufReader::new(input_file);
            let merkle_verify_input: MerkleVerifyInput = serde_json::from_reader(reader)?;
            let verify_merkle_input = || {
                let mut merkle_view_vec = vec![];
                for i in 0..merkle_verify_input.merkle_view.len() {
                    merkle_view_vec.push(MoveValue::U256(U256::from_str(&*merkle_verify_input.merkle_view[i].clone()).unwrap()));
                }
                let merkle_view = MoveValue::Vector(merkle_view_vec);

                let mut initial_merkle_queue_vec = vec![];
                for i in 0..merkle_verify_input.initial_merkle_queue.len() {
                    initial_merkle_queue_vec.push(MoveValue::U256(U256::from_str(&*merkle_verify_input.initial_merkle_queue[i].clone()).unwrap()));
                }
                let initial_merkle_queue = MoveValue::Vector(initial_merkle_queue_vec);

                let height = MoveValue::U256(U256::from_str(&*merkle_verify_input.height.clone()).unwrap());
                let expected_root = MoveValue::U256(U256::from_str(&*merkle_verify_input.expected_root.clone()).unwrap());
                (merkle_view, initial_merkle_queue, height, expected_root)
            };
            let (merkle_view, initial_merkle_queue, height, expected_root) = verify_merkle_input();
            let verify_merkle_input = VerifyMerkleTransactionInput {
                merkle_view,
                initial_merkle_queue,
                height,
                expected_root,
            };

            let sequence_number = config.client.get_account(config.account.address()).await?.into_inner().sequence_number;
            config.account.set_sequence_number(sequence_number);

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

            if !verify_merkle(verify_merkle_cycles, &config, &input_verify_merkle).await? {
                eprintln!("something went wrong!");
                return Ok(());
            }

            if !register_fact_merkle(&config, input_register_fact_merkle).await? {
                eprintln!("something went wrong!");
                return Ok(());
            }
        }

        for i in 1..8 {
            let file_path = format!("./src/data_samples/fri_verify/fri_verify_{}.json", i);
            let input_file = File::open(file_path)?;
            let reader = BufReader::new(input_file);
            let fri_verify_input: FriVerifyInput = serde_json::from_reader(reader)?;
            let verify_fri_input = || {
                let mut proof_vec = vec![];
                for i in 0..fri_verify_input.proof.len() {
                    proof_vec.push(MoveValue::U256(U256::from_str(&*fri_verify_input.proof[i].clone()).unwrap()));
                }
                let proof = MoveValue::Vector(proof_vec);

                let mut fri_queue_vec = vec![];
                for i in 0..fri_verify_input.fri_queue.len() {
                    fri_queue_vec.push(MoveValue::U256(U256::from_str(&*fri_verify_input.fri_queue[i].clone()).unwrap()));
                }
                let fri_queue = MoveValue::Vector(fri_queue_vec);

                let evaluation_point = MoveValue::U256(U256::from_str(&*fri_verify_input.evaluation_point.clone()).unwrap());
                let fri_step_size = MoveValue::U256(U256::from_str(&*fri_verify_input.fri_step_size.clone()).unwrap());
                let expected_root = MoveValue::U256(U256::from_str(&*fri_verify_input.expected_root.clone()).unwrap());
                (proof, fri_queue, evaluation_point, fri_step_size, expected_root)
            };
            let (proof, fri_queue, evaluation_point, fri_step_size, expected_root) = verify_fri_input();
            let verify_merkle_input = VerifyFriTransactionInput {
                proof,
                fri_queue,
                evaluation_point,
                fri_step_size,
                expected_root,
            };

            let (event_init, event_compute, event_register) = verify_fri(&config, verify_merkle_input.clone()).await?;

            let input_init: InitFriGroup = event_init.try_into()?;
            let input_compute: ComputeNextLayer = event_compute.clone().try_into()?;
            let input_register: RegisterFactVerifyFri = event_register.try_into()?;

            init_fri_group(&config, input_init).await?;

            let count_next_layer_cycles_request = config.client.view(&ViewRequest {
                function: EntryFunctionId::from_str(format!("{}::fri_layer::count_next_layer_cycles", config.module_address).as_str()).unwrap(),
                type_arguments: vec![],
                arguments: vec![
                    serde_json::Value::String(config.account.address().to_string()),
                    serde_json::Value::String(input_compute.channel_ptr.to_string()),
                    serde_json::Value::String(input_compute.fri_queue_ptr.to_string()),
                    serde_json::Value::String(input_compute.n_queries.to_string()),
                    serde_json::Value::String(input_compute.fri_ctx.to_string()),
                    serde_json::Value::String(input_compute.fri_coset_size.to_string()),
                ],
            }, None).await?;
            let next_layer_cycles = count_next_layer_cycles_request.into_inner().remove(0).as_str().unwrap().parse::<usize>().unwrap();
            eprintln!("next_layer_cycles = {:#?}", next_layer_cycles);

            compute_next_layer(next_layer_cycles, &config, &input_compute).await?;
            if !simulate_compute_next_layer(&config, &input_compute).await.unwrap() {
                eprintln!("something went wrong!");
                return Ok(());
            }

            let count_verify_merkle_cycles_request = config.client.view(&ViewRequest {
                function: EntryFunctionId::from_str(format!("{}::merkle_verifier::count_verify_merkle_cycles", config.module_address).as_str()).unwrap(),
                type_arguments: vec![],
                arguments: vec![
                    serde_json::Value::String(config.account.address().to_string()),
                    serde_json::Value::String(input_compute.merkle_queue_ptr.to_string()),
                    serde_json::Value::String(input_compute.n_queries.to_string()),
                ],
            }, None).await?;
            let verify_merkle_cycles = count_verify_merkle_cycles_request.into_inner().remove(0).as_str().unwrap().parse::<usize>().unwrap();
            eprintln!("verify_merkle_cycles = {:#?}", verify_merkle_cycles);


            let input_verify_merkle: VerifyMerkle = VerifyMerkle {
                channel_ptr: input_compute.channel_ptr,
                merkle_queue_ptr: input_compute.merkle_queue_ptr,
                expected_root: U256::from_str(&*fri_verify_input.expected_root).unwrap(),
                n_queries: input_compute.n_queries,
            };

            if !verify_merkle(verify_merkle_cycles, &config, &input_verify_merkle).await? {
                eprintln!("something went wrong!");
                return Ok(());
            }

            if !register_fact_fri(&config, input_register, input_compute.n_queries).await? {
                eprintln!("something went wrong!");
                return Ok(());
            }

            eprintln!("verify success! {}", i);
        }

        Ok(())
    }
}


