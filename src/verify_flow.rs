#[cfg(test)]
mod tests {
    use aptos_sdk::move_types::value::MoveValue;
    use aptos_sdk::rest_client::aptos_api_types::Event;

    use crate::config::AppConfig;
    use crate::config::EnvConfig;
    use crate::contracts_caller::helper::str_to_u256;
    use crate::contracts_caller::types::{ComputeNextLayer, InitFriGroup};
    use crate::contracts_caller::verify_fri::init_fri_group::init_fri_group;
    use crate::contracts_caller::verify_fri::verify_fri::verify_fri;
    use crate::data_samples::sample_1::sample1;

    #[tokio::test]
    pub async fn test_a() -> anyhow::Result<()> {
        let (verify_input, root_hash) = sample1();
        let config = AppConfig::from(EnvConfig::new());
        let sequence_number = config.client.get_account(config.account.address()).await?.into_inner().sequence_number;
        config.account.set_sequence_number(sequence_number);


        let (event_init, event_compute) = verify_fri(&config, verify_input).await.expect("E");
        let fri_ctx = str_to_u256(event_init.data.get("fri_ctx").unwrap().as_str().unwrap()).unwrap();
        let input_init = InitFriGroup {
            fri_ctx: MoveValue::U256(fri_ctx)
        };

        let input_compute: ComputeNextLayer = event_compute.clone().try_into()?;

        init_fri_group(&config, input_init).await.expect("E");
        let mut n_queries: Event;

        // loop {
        //     n_queries = compute_next_layer(10, &config, &input_compute).await.expect("E");
        //     // if !compute_next_layer_view(&config).await.unwrap() {
        //     break;
        //     // }
        // }

        // let input_verify_merkle = VerifyMerkle {
        //     channel_ptr: str_to_u256(event_compute.data.get("channel_ptr").unwrap().as_str().unwrap()),
        //     merkle_queue_ptr: str_to_u256(event_compute.data.get("merkle_queue_ptr").unwrap().as_str().unwrap()),
        //     root: str_to_u256(root_hash.as_str()),
        //     n_queries: str_to_u256(n_queries.data.get("n_queries").unwrap().as_str().unwrap()),
        // };
        //
        // loop {
        //     verify_merkle(&config, &input_verify_merkle).await.expect("E");
        //     // if !verify_merkle_view(&config).await.unwrap() {
        //         break;
        //     // }
        //     println!("merkle_verifier {}", true);
        // }

        Ok(())
    }
}


