use std::str::FromStr;

use anyhow::Result;
use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::LocalAccount;
use dotenv::dotenv;

pub struct EnvConfig {
    pub node_url: String,
    pub private_key: String,
    pub module_address: String,
    pub chain_id: String,
}

pub fn get_env_var(key: &str) -> Result<String> {
    std::env::var(key).map_err(|e| e.into())
}

pub fn get_env_var_or_panic(key: &str) -> String {
    get_env_var(key).unwrap_or_else(|e| panic!("Failed to get env var {}: {}", key, e))
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvConfig {
    pub fn new() -> Self {
        dotenv().expect("Failed to load .env file");
        let node_url = get_env_var_or_panic("APTOS_NODE_URL");
        let private_key = get_env_var_or_panic("APTOS_PRIVATE_KEY");
        let module_address = get_env_var_or_panic("APTOS_MODULE_ADDRESS");
        let chain_id = get_env_var_or_panic("CHAIN_ID");
        EnvConfig {
            chain_id,
            node_url,
            private_key,
            module_address,
        }
    }
}

pub struct AppConfig {
    pub client: Client,
    pub account: LocalAccount,
    pub module_address: AccountAddress,
    pub chain_id: ChainId,
}

impl From<EnvConfig> for AppConfig {
    fn from(config: EnvConfig) -> Self {
        let client = Client::new(config.node_url.parse().unwrap());
        let account = LocalAccount::from_private_key(&config.private_key, 0).unwrap();
        let module_address = config
            .module_address
            .parse()
            .expect("Invalid module address");
        let chain_id = ChainId::from_str(&config.chain_id).expect("Invalid chain id");

        AppConfig {
            client,
            account,
            module_address,
            chain_id,
        }
    }
}
