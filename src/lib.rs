mod config;

use anyhow::{Context, Result};
use aptos_sdk::transaction_builder;
use aptos_sdk::types::{LocalAccount};
use aptos_sdk::crypto::{ed25519::Ed25519PrivateKey};
use once_cell::sync::Lazy;
use url::Url;
use std::str::FromStr;

#[tokio::main]
async fn main() {

}