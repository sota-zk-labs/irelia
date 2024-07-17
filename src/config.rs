pub struct AptosVerifierConfig {
    pub node_url: String,
    pub private_key: String,
    pub account_address: String,
}
impl AptosVerifierConfig {
    pub fn new() -> Self {
        let node_url = std::env::var("APTOS_NODE_URL").unwrap_or("https://fullnode.devnet.aptoslabs.com".to_string());
        let private_key = std::env::var("APTOS_PRIVATE_KEY").unwrap();
        let account_address = std::env::var("APTOS_ACCOUNT_ADDRESS").unwrap();
        AptosVerifierConfig {
            node_url,
            private_key,
            account_address,
        }
    }
}