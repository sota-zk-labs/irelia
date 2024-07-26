use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::rest_client::aptos_api_types::{AptosErrorCode, ViewFunction};
use aptos_sdk::rest_client::error::RestError;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::config::AppConfig;
use crate::contracts::helper::{str_to_bool, build_simulated_transaction, build_transaction, get_error_code};
use crate::contracts::types::VerifyMerkle;

pub async fn verify_merkle(config: &AppConfig, data: &VerifyMerkle) -> anyhow::Result<()> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("merkle_verifier").unwrap()),
            Identifier::new("verify_merkle").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    data.channel_ptr.clone(),
                    data.merkle_queue_ptr.clone(),
                    data.root.clone(),
                    data.n_queries.clone(),
                ]
            ),
        ));
    let tx = build_simulated_transaction(payload.clone(), &config.account, config.chain_id);
    let simulate = config.client.simulate(&tx).await.unwrap().into_inner();
    let vm_status = simulate.get(0).unwrap().info.vm_status.as_str();
    eprintln!("get_error_code(vm_status) = {:#?}", get_error_code(vm_status));
    if (simulate.get(0).unwrap().info.success ) {
        let tx = build_transaction(payload, &config.account, config.chain_id);
        let txd = config.client.submit_and_wait(&tx).await?.into_inner();
        println!("verify_merkle {:?}", txd);
    } else if  { get_error_code(vm_status) == 3  } {
        println!("Verify Merkle Success");
    } else {
        println!("Verify Merkle Failed with error code: {:#?}", get_error_code(vm_status));
    }
    Ok(())
}

pub async fn verify_merkle_view(config: &AppConfig) -> anyhow::Result<bool> {
    let view_payload = ViewFunction {
        module: ModuleId::new(config.module_address, Identifier::new("merkle_verifier").unwrap()),
        function: Identifier::new("check_in_mloop").unwrap(),
        ty_args: vec![],
        args: serialize_values(&vec![MoveValue::Address(config.account.address())]),
    };
    let data = config.client.view_bcs_with_json_response(&view_payload, None).await.unwrap().into_inner();



    let data_str = format!("{:?}", data[0]);
    eprintln!("data_str = {:#?}", data_str);
    Ok(str_to_bool(&data_str))
}