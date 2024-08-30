use std::str::FromStr;
use std::time::SystemTime;

use anyhow::{ensure, Error};
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::rest_client::aptos_api_types::{Event, MoveType};
use aptos_sdk::rest_client::Transaction;
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::transaction::{EntryFunction, SignedTransaction, TransactionPayload};
use aptos_sdk::types::LocalAccount;
use log::info;
use rand_core::OsRng;

use crate::config::AppConfig;
use crate::error::CoreError;

#[inline]
pub fn str_to_u256(s: &str) -> Result<U256, CoreError> {
    U256::from_str(s).map_err(|e| e.into())
}

#[inline]
pub fn str_to_u64(s: &str) -> Result<u64, CoreError> {
    u64::from_str(s).map_err(|e| e.into())
}

pub fn build_transaction(
    payload: TransactionPayload,
    sender: &LocalAccount,
    chain_id: ChainId,
) -> SignedTransaction {
    let i = sender.increment_sequence_number();
    let tx = TransactionBuilder::new(
        payload,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 60,
        chain_id,
    )
    .sender(sender.address())
    .sequence_number(i)
    .max_gas_amount(200000)
    .gas_unit_price(100)
    .build();
    sender.sign_transaction(tx)
}

pub fn build_simulated_transaction(
    payload: TransactionPayload,
    sender: &LocalAccount,
    chain_id: ChainId,
) -> SignedTransaction {
    let i = sender.sequence_number();
    let tx = TransactionBuilder::new(
        payload,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 60,
        chain_id,
    )
    .sender(sender.address())
    .sequence_number(i)
    .max_gas_amount(10000)
    .gas_unit_price(100)
    .build();
    let mut r = OsRng;
    tx.sign(
        LocalAccount::generate(&mut r).private_key(),
        sender.public_key().clone(),
    )
    .expect("signing a txn can't fail")
    .into_inner()
}

pub fn get_event_from_transaction(
    transaction: &Transaction,
    event_type: MoveType,
) -> anyhow::Result<&Event> {
    let event = match transaction {
        Transaction::UserTransaction(txn) => txn.events.iter().find(|s| s.typ == event_type),
        Transaction::BlockMetadataTransaction(_) => None,
        Transaction::PendingTransaction(_) => None,
        Transaction::GenesisTransaction(_) => None,
        Transaction::StateCheckpointTransaction(_) => None,
        Transaction::BlockEpilogueTransaction(_) => None,
        Transaction::ValidatorTransaction(_) => None,
    };
    event.ok_or(Error::new(CoreError::NotFound))
}

pub async fn send_tx(
    config: &AppConfig,
    module_name: &str,
    fn_name: &str,
    args: &Vec<MoveValue>,
) -> anyhow::Result<Transaction> {
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(config.module_address, Identifier::new(module_name)?),
        Identifier::new(fn_name)?,
        vec![],
        serialize_values(args),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let transaction_info = transaction.transaction_info()?;
    info!(
        "Finished: {} {}; gas used: {}",
        fn_name,
        transaction_info.hash.to_string(),
        transaction_info.gas_used
    );
    ensure!(
        transaction_info.success,
        CoreError::TransactionNotSucceed(transaction_info.hash.to_string())
    );
    Ok(transaction)
}
