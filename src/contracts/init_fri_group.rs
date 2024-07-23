use std::hash::Hash;

use aptos_sdk::crypto::HashValue;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use crate::AptosClient;
use crate::config::AptosVerifierConfig;
use crate::contracts::helper::{init_config, transaction_builder};
use crate::contracts::types::InitFriGroup;

pub async fn init_fri_group(data: InitFriGroup) -> anyhow::Result<(HashValue)> {
    let config = AptosClient::from(AptosVerifierConfig::new());
    let client = config.client;
    let account = config.account;
    let module_address = config.module_address;
    let account_sequence =  client.get_account(account.address()).await?.into_inner().sequence_number;
    account.set_sequence_number(account_sequence);
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(module_address, Identifier::new("fri_layer").unwrap()),
            Identifier::new("init_fri_group").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    data.fri_ctx
                ]
            ),
        ));
    let tx = transaction_builder(payload, &account, ChainId::testnet());
    let txn = account.sign_transaction(tx);
    let txd = client.submit(&txn).await?.into_inner().hash;
    println!("Init Fri Group {}", txd);
    Ok(HashValue::from((txd)))
}