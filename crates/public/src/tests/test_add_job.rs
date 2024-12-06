use std::fs;

use reqwest::Client;
use serde_json::{json, Value};
use tokio;
use uuid::Uuid;

use crate::options::Options;

#[tokio::test]
async fn test_add_job() {
    let client = Client::new();

    let config_content =
        fs::read_to_string("./config/00-default.toml").expect("Failed to read config file");

    let options: Options = toml::from_str(&config_content).expect("Failed to parse config file");

    let base_url = format!(
        "http://{}:{}",
        options.server.url.as_str(),
        options.server.port
    );

    let cairo_pie = fs::read_to_string("./src/assets/test_data/encoded_cairo_pie.txt").unwrap();

    test_incorrect_layout(client.clone(), base_url.clone(), cairo_pie.clone()).await;
    println!("✅ test_incorrect_layout completed");

    test_additional_bad_flag(client.clone(), base_url.clone(), cairo_pie.clone()).await;
    println!("✅ test_additional_bad_flag completed");

    test_no_cairo_job_id(client.clone(), base_url.clone(), cairo_pie.clone()).await;
    println!("✅ test_no_cairo_job_id completed");

    test_incorrect_offchain_proof(client.clone(), base_url.clone(), cairo_pie.clone()).await;
    println!("✅ test_incorrect_offchain_proof completed");

    test_successfully(client.clone(), base_url.clone(), cairo_pie.clone()).await;
    println!("✅ test_successfully completed");
}

async fn test_incorrect_layout(client: Client, base_url: String, cairo_pie: String) {
    let url =
        format!(
        "{}/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
        base_url, Uuid::new_v4(), Uuid::new_v4(), true, "smal"
    );
    let correct_body = cairo_pie.to_string();
    let expected = json!(
        {
            "code": "500",
            "message": "Internal server error"
        }
    );
    let res = post_request(client, url, correct_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_additional_bad_flag(client: Client, base_url: String, cairo_pie: String) {
    let url = format!(
        "{}/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}&bla={}",
        base_url, Uuid::new_v4(), Uuid::new_v4(), true, "small", true
    );
    let correct_body = cairo_pie.to_string();
    let expected = json!(
        {"code" : "JOB_RECEIVED_SUCCESSFULLY"}
    );
    let res = post_request(client, url, correct_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_no_cairo_job_id(client: Client, base_url: String, cairo_pie: String) {
    let url = format!(
        "{}/v1/gateway/add_job?customer_id={}&offchain_proof={}&proof_layout={}",
        base_url,
        Uuid::new_v4(),
        true,
        "small"
    );
    let correct_body = cairo_pie.to_string();
    let expected = json!(
        {
            "code": "500",
            "message": "Internal server error"
        }
    );
    let res = post_request(client, url, correct_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_incorrect_offchain_proof(client: Client, base_url: String, cairo_pie: String) {
    let url =
        format!(
        "{}/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
        base_url, Uuid::new_v4(), Uuid::new_v4(), false, "small"
    );
    let correct_body = cairo_pie.to_string();
    let expected = json!(
        {
            "code": "500",
            "message": "Internal server error"
        }
    );
    let res = post_request(client, url, correct_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_successfully(client: Client, base_url: String, cairo_pie: String) {
    let url =
        format!(
        "{}/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
        base_url, Uuid::new_v4(), Uuid::new_v4(), true, "small"
    );

    let correct_body = cairo_pie.to_string();

    let expected = json!(
        {"code" : "JOB_RECEIVED_SUCCESSFULLY"}
    );
    let res = post_request(client, url, correct_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn post_request(client: Client, url: String, body: String) -> Value {
    client
        .post(&url)
        .body(body)
        .send()
        .await
        .expect("Failed to send POST request")
        .json::<Value>()
        .await
        .expect("Failed to parse response body as JSON")
}