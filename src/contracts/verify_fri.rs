use std::str::FromStr;
use std::time::SystemTime;
use aptos_sdk::crypto::HashValue;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::rest_client::aptos_api_types::{MoveType, ViewFunction};
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};

use crate::AptosClient;
use crate::config::AptosVerifierConfig;
use crate::contracts::event_tracker::EventTracker;
use crate::contracts::types::Verify;

pub async fn verify_fri(data: Verify) -> anyhow::Result<()> {
    let config = AptosClient::from(AptosVerifierConfig::new());
    let client = config.client;
    let account = config.account;
    let module_address = config.module_address;
    let account_sequence =  client.get_account(account.address()).await?.into_inner().sequence_number;
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
        ChainId::testnet(),
    )
        .sender(account.address())
        .sequence_number(account.sequence_number())
        .max_gas_amount(10000)
        .gas_unit_price(100)
        .build();

    let txn = account.sign_transaction(tx);
    let txd = client.submit(&txn).await?.into_inner().hash;

    let tx = client.get_transaction_by_hash(HashValue::from(txd)).await?.into_inner();
    eprintln!("txd = {:#?}", txd);
    eprintln!("account.address() = {:#?}", account.address());

    let mut e = EventTracker::new(
        client,
        account.address(),
        MoveType::from_str("0xa3ef536178c53b3c989a7f9b2750d91ad5c753975467d6d01eb2aef4bd7945ec::fri_statement::FriCtx").unwrap(),
        3,
    );

    let event = e.latest_event().await.unwrap();
    eprintln!("event = {:#?}", event);
    eprintln!("event.data = {:#?}", event.data);
    Ok(())
}
