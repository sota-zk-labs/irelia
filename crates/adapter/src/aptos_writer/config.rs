use std::str::FromStr;
use std::sync::Arc;

use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::LocalAccount;

pub struct Config {
    pub node_url: String,
    pub private_key: String,
    pub aptos_verifier_address: String,
    pub chain_id: String,
}

#[derive(Clone)]
pub struct AppConfig {
    pub client: Client,
    pub account: Arc<LocalAccount>,
    pub verifier_address: AccountAddress,
    pub chain_id: ChainId,
}

impl From<Config> for AppConfig {
    fn from(config: Config) -> Self {
        let client = Client::new(config.node_url.parse().unwrap());
        let account = Arc::new(LocalAccount::from_private_key(&config.private_key, 0).unwrap());
        let verifier_address = config
            .aptos_verifier_address
            .parse()
            .expect("Invalid verifier address");
        let chain_id = ChainId::from_str(&config.chain_id).expect("Invalid chain id");
        AppConfig {
            client,
            account,
            verifier_address,
            chain_id,
        }
    }
}
