use std::str::FromStr;
use std::time::SystemTime;
use anyhow::anyhow;
use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;
use aptos_sdk::rest_client::aptos_api_types::Address;
use aptos_sdk::rest_client::Client;
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::account_address::AccountAddress;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::LocalAccount;
use aptos_sdk::types::transaction::{EntryFunction, RawTransaction, TransactionPayload};
use crate::AptosClient;
use crate::config::AptosVerifierConfig;

pub fn str_to_u256(s: &str) -> MoveValue {
    let u256_value = U256::from_str(s).unwrap();
    MoveValue::U256(u256_value)
}

pub fn str_to_bool(s: &str) -> bool {
    let bool_str = s.trim_start_matches("Bool(").trim_end_matches(")");
    bool::from_str(bool_str).unwrap()
}

pub fn transaction_builder(payload: TransactionPayload, sender: &LocalAccount, chain_id: ChainId) -> RawTransaction{
    TransactionBuilder::new(
        payload,
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() +60,
        chain_id
    )
        .sender(sender.address())
        .sequence_number(sender.sequence_number())
        .max_gas_amount(10000)
        .gas_unit_price(100)
        .build()
}

pub async fn init_config() -> anyhow::Result<(Client,LocalAccount,AccountAddress)> {
    let config = AptosClient::from(AptosVerifierConfig::new());
    let client = config.client;
    let account = config.account;
    let module_address = config.module_address;
    let account_sequence =  client.get_account(account.address()).await?.into_inner().sequence_number;
    account.set_sequence_number(account_sequence);
    Ok((client, account, module_address))
}