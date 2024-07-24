use aptos_sdk::rest_client::aptos_api_types::Event;

use verifier_onchain_services::config::{AppConfig, EnvConfig};
use verifier_onchain_services::contracts::compute_next_layer::{compute_next_layer, compute_next_layer_view};
use verifier_onchain_services::contracts::helper::str_to_u256;
use verifier_onchain_services::contracts::init_fri_group::init_fri_group;
use verifier_onchain_services::contracts::types::{ComputeNextLayer, InitFriGroup, VerifyMerkle};
use verifier_onchain_services::contracts::verify_fri::verify_fri;
use verifier_onchain_services::contracts::verify_merkle::{verify_merkle, verify_merkle_view};
use verifier_onchain_services::data_samples::sample_1::sample1;

#[tokio::main]
async fn main() {
    let (verify_input, root_hash) = sample1();
    let config = AppConfig::from(EnvConfig::new());
    let sequence_number = config.client.get_account(config.account.address()).await.unwrap().into_inner().sequence_number;
    config.account.set_sequence_number(sequence_number);

    let (event_init, event_compute) = verify_fri(&config, verify_input).await.expect("E");
    let fri_ctx = str_to_u256(event_init.data.get("fri_ctx").unwrap().as_str().unwrap());
    let input_init = InitFriGroup {
        fri_ctx
    };

    let input_compute = ComputeNextLayer {
        channel_ptr: str_to_u256(event_compute.data.get("channel_ptr").unwrap().as_str().unwrap()),
        evaluation_point: str_to_u256(event_compute.data.get("evaluation_point").unwrap().as_str().unwrap()),
        fri_coset_size: str_to_u256(event_compute.data.get("fri_coset_size").unwrap().as_str().unwrap()),
        fri_ctx: str_to_u256(event_compute.data.get("fri_ctx").unwrap().as_str().unwrap()),
        fri_queue_ptr: str_to_u256(event_compute.data.get("fri_queue_ptr").unwrap().as_str().unwrap()),
        merkle_queue_ptr: str_to_u256(event_compute.data.get("merkle_queue_ptr").unwrap().as_str().unwrap()),
        n_queries: str_to_u256(event_compute.data.get("n_queries").unwrap().as_str().unwrap()),
    };

    init_fri_group(&config, input_init).await.expect("E");
    let mut n_queries: Event;

    loop {
        n_queries = compute_next_layer(&config, &input_compute).await.expect("E");
        if !compute_next_layer_view(&config).await.unwrap() {
            break;
        }
    }

    let input_verify_merkle = VerifyMerkle {
        channel_ptr: str_to_u256(event_compute.data.get("channel_ptr").unwrap().as_str().unwrap()),
        merkle_queue_ptr: str_to_u256(event_compute.data.get("merkle_queue_ptr").unwrap().as_str().unwrap()),
        root: str_to_u256(root_hash.as_str()),
        n_queries: str_to_u256(n_queries.data.get("n_queries").unwrap().as_str().unwrap()),
    };

    loop {
        verify_merkle(&config, &input_verify_merkle).await.expect("E");
        if !verify_merkle_view(&config).await.unwrap() {
            break;
        }
        println!("merkle_verifier {}", true);
    }
    eprintln!("done");
}
