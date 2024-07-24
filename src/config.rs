use dotenv::dotenv;
use color_eyre::Result;
pub struct AptosVerifierConfig {
    pub node_url: String,
    pub private_key: String,
    pub account_address: String,
    pub module_address: String,
    pub chain_id: String,
}

pub fn get_env_var(key: &str) -> Result<String> {
    std::env::var(key).map_err(|e| e.into())
}

pub fn get_env_var_or_panic(key: &str) -> String {
    get_env_var(key).unwrap_or_else(|e| panic!("Failed to get env var {}: {}", key, e))
}

impl AptosVerifierConfig {
    pub fn new() -> Self {
        dotenv().ok().expect("Failed to load .env.example file");
        let node_url = get_env_var_or_panic("APTOS_NODE_URL");
        let private_key = get_env_var_or_panic("APTOS_PRIVATE_KEY");
        let account_address = get_env_var_or_panic("APTOS_ACCOUNT_ADDRESS");
        let module_address = get_env_var_or_panic("APTOS_MODULE_ADDRESS");
        let chain_id = get_env_var_or_panic("CHAIN_ID");
        AptosVerifierConfig {
            chain_id,
            node_url,
            private_key,
            account_address,
            module_address,
        }
    }
}