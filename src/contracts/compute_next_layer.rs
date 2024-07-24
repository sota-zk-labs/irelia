use std::str::FromStr;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::{MoveValue, serialize_values};
use aptos_sdk::rest_client::aptos_api_types::{Event, MoveType, ViewFunction};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use crate::config::AppConfig;
use crate::contracts::helper::{str_to_bool, build_transaction, get_event_from_transaction};
use crate::contracts::types::ComputeNextLayer;

pub async fn compute_next_layer(config: &AppConfig, data: &ComputeNextLayer) -> anyhow::Result<Event> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_layer").unwrap()),
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
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();

    let event_type = MoveType::from_str(&format!("{}::fri_layer::NQueries", config.module_address)).unwrap();
    get_event_from_transaction(transaction, event_type).await
}

pub async fn compute_next_layer_view(config: &AppConfig) -> anyhow::Result<bool> {
    let view_payload = ViewFunction {
        module: ModuleId::new(config.module_address, Identifier::new("fri_layer").unwrap()),
        function: Identifier::new("check_in_loop").unwrap(),
        ty_args: vec![],
        args: serialize_values(&vec![MoveValue::Address(config.account.address())]),
    };
    //TODO: Make to_bool function
    let data = config.client.view_bcs_with_json_response(&view_payload, None).await.unwrap().into_inner();
    let data_str = format!("{:?}", data[0]);
    Ok(str_to_bool(&data_str))
}