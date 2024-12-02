use reqwest::Client;
use serde_json::{json, Value};
use tokio;
use tokio_postgres::NoTls;
use uuid::Uuid;
#[tokio::test]
async fn test_add_job() {
    let client = Client::new();

    // Set up the database
    setup_database().await;
    println!("✅ Database setup completed");

    test_faulty_cairo_pie(client.clone()).await;
    println!("✅ test_faulty_cairo_pie completed");

    test_incorrect_layout(client.clone()).await;
    println!("✅ test_incorrect_layout completed");

    test_additional_bad_flag(client.clone()).await;
    println!("✅ test_additional_bad_flag completed");

    test_no_cairo_job_id(client.clone()).await;
    println!("✅ test_no_cairo_job_id completed");

    test_incorrect_offchain_proof(client.clone()).await;
    println!("✅ test_incorrect_offchain_proof completed");

    test_successfully(client.clone()).await;
    println!("✅ test_faulty_cairo_pie completed");
}

async fn test_faulty_cairo_pie(client: Client) {
    let url = format!(
        "http://localhost:8000/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
        Uuid::new_v4().to_string(), Uuid::new_v4().to_string(), true, "small".to_string()
    );
    let incorrect_body = json!(
        {
            "action": "add_job",
            "request": {
                "cairo_pie": ""
            }
        }
    );
    let expected = json!(
        {"code" : "JOB_RECEIVED_SUCCESSFULLY"}
    );
    let res = post_request(client, url, incorrect_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_incorrect_layout(client: Client) {
    let url = format!(
        "http://localhost:8000/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
        Uuid::new_v4().to_string(), Uuid::new_v4().to_string(), true, "smal".to_string()
    );
    let correct_body = json!(
        {
            "action": "add_job",
            "request": {
                "cairo_pie": "/home/andrew/workspace/irelia/crates/\
                    adapter/src/prover/test_samples/fibonacci_with_output.zip"
            }
        }
    );
    let expected = json!(
        {
            "code": "500",
            "message": "Internal server error"
        }
    );
    let res = post_request(client, url, correct_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_additional_bad_flag(client: Client) {
    let url = format!(
        "http://localhost:8000/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}&bla={}",
        Uuid::new_v4().to_string(), Uuid::new_v4().to_string(), true, "small".to_string(), true
    );
    let correct_body = json!(
        {
            "action": "add_job",
            "request": {
                "cairo_pie": "/home/andrew/workspace/irelia/crates/\
                    adapter/src/prover/test_samples/fibonacci_with_output.zip"
            }
        }
    );
    let expected = json!(
        {"code" : "JOB_RECEIVED_SUCCESSFULLY"}
    );
    let res = post_request(client, url, correct_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_no_cairo_job_id(client: Client) {
    let url =
        format!(
        "http://localhost:8000/v1/gateway/add_job?customer_id={}&offchain_proof={}&proof_layout={}",
        Uuid::new_v4().to_string(), true, "small".to_string()
    );
    let correct_body = json!(
        {
            "action": "add_job",
            "request": {
                "cairo_pie": "/home/andrew/workspace/irelia/crates/\
                    adapter/src/prover/test_samples/fibonacci_with_output.zip"
            }
        }
    );
    let expected = json!(
        {
            "code": "500",
            "message": "Internal server error"
        }
    );
    let res = post_request(client, url, correct_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_incorrect_offchain_proof(client: Client) {
    let url = format!(
        "http://localhost:8000/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
        Uuid::new_v4().to_string(), Uuid::new_v4().to_string(), false, "small".to_string()
    );
    let correct_body = json!(
        {
            "action": "add_job",
            "request": {
                "cairo_pie": "/home/andrew/workspace/irelia/crates/\
                    adapter/src/prover/test_samples/fibonacci_with_output.zip"
            }
        }
    );
    let expected = json!(
        {
            "code": "500",
            "message": "Internal server error"
        }
    );
    let res = post_request(client, url, correct_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_successfully(client: Client) {
    let url = format!(
        "http://localhost:8000/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
        Uuid::new_v4().to_string(), Uuid::new_v4().to_string(), true, "small".to_string()
    );
    let correct_body = json!(
        {
            "action": "add_job",
            "request": {
                "cairo_pie": "/home/andrew/workspace/irelia/crates/\
                    adapter/src/prover/test_samples/fibonacci_with_output.zip"
            }
        }
    );
    let expected = json!(
        {"code" : "JOB_RECEIVED_SUCCESSFULLY"}
    );
    let res = post_request(client, url, correct_body).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn post_request(client: Client, url: String, body: Value) -> Value {
    client
        .post(&url)
        .json(&body)
        .send()
        .await
        .expect("Failed to send POST request")
        .json::<Value>()
        .await
        .expect("Failed to parse response body as JSON")
}

async fn setup_database() {
    let (client, connection) = tokio_postgres::connect(
        "postgres://postgres:changeme@localhost:5432/postgres",
        NoTls,
    )
    .await
    .expect("Failed to connect to database");

    // Spawn the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // SQL to drop and recreate the table
    let reset_queries = r#"
        DROP TABLE IF EXISTS jobs;

        CREATE TABLE jobs (
            id UUID PRIMARY KEY,
            customer_id VARCHAR NOT NULL,
            cairo_job_key VARCHAR NOT NULL,
            status VARCHAR NOT NULL,
            invalid_reason VARCHAR NOT NULL,
            error_log VARCHAR NOT NULL,
            validation_done BOOLEAN NOT NULL,
            created_on TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_on TIMESTAMP NOT NULL DEFAULT NOW()
        );
    "#;

    client
        .batch_execute(reset_queries)
        .await
        .expect("Failed to reset database");
}
