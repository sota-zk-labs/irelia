use std::str::FromStr;
use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::crypto::ValidCryptoMaterialStringExt;
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::{AccountKey, LocalAccount};
use aptos_sdk::types::account_address::AccountAddress;
use aptos_sdk::types::chain_id::ChainId;
use crate::config::AptosVerifierConfig;

mod contracts;

pub mod config;
mod verify_flow;

pub struct AptosClient {
    pub client: Client,
    pub account: LocalAccount,
    pub module_address: AccountAddress,
    pub chain_id: ChainId,
}

impl From<AptosVerifierConfig> for AptosClient {
    fn from(value: AptosVerifierConfig) -> Self {
        let client = Client::new(value.node_url.parse().unwrap());
        let private_key = Ed25519PrivateKey::from_encoded_string(&*value.private_key).expect("Failed to parse private key");
        let account_key = AccountKey::from(private_key);
        let account_address = value.account_address.parse().expect("Invalid account address");
        let account = LocalAccount::new(account_address, account_key, 186);
        let module_address = value.module_address.parse().expect("Invalid module address");
        let chain_id = ChainId::from_str(&value.chain_id).expect("Invalid chain id");
        AptosClient {
            client,
            account,
            module_address,
            chain_id,
        }
    }
}


