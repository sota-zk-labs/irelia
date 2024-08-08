use std::str::FromStr;

use aptos_sdk::rest_client::aptos_api_types::{EntryFunctionId, ViewRequest};

use verifier_onchain_services::config::{AppConfig, EnvConfig};
use verifier_onchain_services::contracts_caller::types::{RegisterFactVerifyMerkle, VerifyMerkle};
use verifier_onchain_services::contracts_caller::verify_fri::verify_merkle::verify_merkle;
use verifier_onchain_services::contracts_caller::verify_merkle::merkle_statement::verify_merkle_statement;
use verifier_onchain_services::contracts_caller::verify_merkle::register_fact_merkle::register_fact_merkle;
use verifier_onchain_services::data_samples::merkle_verify::merkle_verify_1::verify_merkle_1;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let verify_merkle_input= verify_merkle_1();
    let config = AppConfig::from(EnvConfig::new());
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

    eprintln!("verify success!");





    // let (event_init, event_compute) = verify_fri(&config, verify_input).await?;
    //
    // let input_init: InitFriGroup = event_init.try_into()?;
    //
    // let input_compute: ComputeNextLayer = event_compute.clone().try_into()?;
    // init_fri_group(&config, input_init).await?;
    //
    // let count_next_layer_cycles_request = config.client.view(&ViewRequest {
    //     function: EntryFunctionId::from_str(format!("{}::fri_layer::count_next_layer_cycles", config.module_address).as_str()).unwrap(),
    //     type_arguments: vec![],
    //     arguments: vec![
    //         serde_json::Value::String(config.account.address().to_string()),
    //         serde_json::Value::String(input_compute.channel_ptr.to_string()),
    //         serde_json::Value::String(input_compute.fri_queue_ptr.to_string()),
    //         serde_json::Value::String(input_compute.n_queries.to_string()),
    //         serde_json::Value::String(input_compute.fri_ctx.to_string()),
    //         serde_json::Value::String(input_compute.fri_coset_size.to_string()),
    //     ],
    // }, None).await?;
    // let next_layer_cycles = count_next_layer_cycles_request.into_inner().remove(0).as_str().unwrap().parse::<usize>().unwrap();
    // eprintln!("next_layer_cycles = {:#?}", next_layer_cycles);
    //
    // compute_next_layer(next_layer_cycles, &config, &input_compute).await.unwrap();
    // if !simulate_compute_next_layer(&config, &input_compute).await.unwrap() {
    //     eprintln!("something went wrong!");
    //     return Ok(());
    // }
    //
    // let count_verify_merkle_cycles_request = config.client.view(&ViewRequest {
    //     function: EntryFunctionId::from_str(format!("{}::merkle_verifier::count_verify_merkle_cycles", config.module_address).as_str()).unwrap(),
    //     type_arguments: vec![],
    //     arguments: vec![
    //         serde_json::Value::String(config.account.address().to_string()),
    //         serde_json::Value::String(input_compute.merkle_queue_ptr.to_string()),
    //         serde_json::Value::String(input_compute.n_queries.to_string()),
    //     ],
    // }, None).await?;
    // let verify_merkle_cycles = count_verify_merkle_cycles_request.into_inner().remove(0).as_str().unwrap().parse::<usize>().unwrap();
    // eprintln!("verify_merkle_cycles = {:#?}", verify_merkle_cycles);
    //
    // if !verify_merkle(verify_merkle_cycles, &config, &input_compute, root_hash.as_str()).await? {
    //     eprintln!("something went wrong!");
    //     return Ok(());
    // }
    //
    // eprintln!("verify success!");
    Ok(())
}
