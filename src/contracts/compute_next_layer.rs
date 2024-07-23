use std::str::FromStr;

use aptos_sdk::crypto::HashValue;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::rest_client::aptos_api_types::{MoveType, VersionedEvent, ViewFunction};
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use crate::AptosClient;
use crate::config::AptosVerifierConfig;
use crate::contracts::event_tracker::EventTracker;
use crate::contracts::helper::{init_config, str_to_bool, transaction_builder};
use crate::contracts::types::ComputeNextLayer;

pub async fn compute_next_layer(data: &ComputeNextLayer) -> anyhow::Result<(VersionedEvent)> {
    let config = AptosClient::from(AptosVerifierConfig::new());
    let client = config.client;
    let account = config.account;
    let module_address = config.module_address;
    let account_sequence =  client.get_account(account.address()).await?.into_inner().sequence_number;
    account.set_sequence_number(account_sequence);
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(module_address, Identifier::new("fri_layer").unwrap()),
            Identifier::new("compute_next_layer").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    data.channel_ptr.clone(),
                    data.fri_queue_ptr.clone(),
                    data.merkle_queue_ptr.clone(),
                    data.n_queries.clone(),
                    data.fri_ctx.clone(),
                    data.evaluation_point.clone(),
                    data.fri_coset_size.clone(),
                ]
            ),
        ));
    let tx = transaction_builder(payload, &account, ChainId::testnet());
    let txn = account.sign_transaction(tx);
    let txd = client.submit(&txn).await?.into_inner().hash;
    println!("Compute Next Layer {}", txd);
    let mut n_queries = EventTracker::new(
        client.clone(),
        account.address(),
        MoveType::from_str(&(module_address.to_string() + "::fri_layer::NQueries")).unwrap(),
        3,
    );
    let event = n_queries.latest_event().await.unwrap();
    Ok((event))
}

pub async fn compute_next_layer_view() -> anyhow::Result<(bool)> {
    let (client, account, module_address) = init_config().await.expect("Error initializing config");
    let view_payload = ViewFunction {
        module: ModuleId::new(module_address, Identifier::new("fri_layer").unwrap()),
        function: Identifier::new("check_in_loop").unwrap(),
        ty_args: vec![],
        args: serialize_values(&vec![MoveValue::Address(account.address())]),
    };
    //TODO: Make to_bool function
    let data = client.view_bcs_with_json_response(&view_payload, None).await.unwrap().into_inner();
    let data_str = format!("{:?}", data[0]);
    Ok(str_to_bool(&data_str))
}