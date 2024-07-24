use std::str::FromStr;
use std::time::SystemTime;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::rest_client::aptos_api_types::{MoveType, VersionedEvent};
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::AptosClient;
use crate::config::AptosVerifierConfig;
use crate::contracts::event_tracker::EventTracker;
use crate::contracts::types::Verify;

pub async fn verify_fri(data: Verify) -> anyhow::Result<(VersionedEvent,VersionedEvent)> {
    let config = AptosClient::from(AptosVerifierConfig::new());
    let client = config.client;
    let account = config.account;
    let module_address = config.module_address;
    let account_sequence = client.get_account(account.address()).await?.into_inner().sequence_number;
    account.set_sequence_number(account_sequence);
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(module_address, Identifier::new("fri_statement").unwrap()),
            Identifier::new("verify_fri").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    data.proof,
                    data.fri_queue,
                    data.evaluation_point,
                    data.fri_step_size,
                    data.expected_root,
                ]
            ),
        ));
    let tx = TransactionBuilder::new(
        payload,
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + 60,
        config.chain_id,
    )
        .sender(account.address())
        .sequence_number(account.sequence_number())
        .max_gas_amount(10000)
        .gas_unit_price(100)
        .build();

    let txn = account.sign_transaction(tx);
    let txd = client.submit(&txn).await?.into_inner().hash;
    println!("Verify FRI: {}", txd);

    let mut fri_ctx = EventTracker::new(
        client.clone(),
        account.address(),
        MoveType::from_str(&format!("{module_address}::fri_statement::FriCtx")).unwrap(),
        3,
    );

    let mut compute_next_layer = EventTracker::new(
        client.clone(),
        account.address(),
        MoveType::from_str(&format!("{module_address}::fri_statement::ComputeNextLayer")).unwrap(),
        4
    );
    let event = fri_ctx.latest_event().await.unwrap();
    let event_compute = compute_next_layer.latest_event().await.unwrap();

    Ok((event, event_compute))
}