use e2e_tests::postgres::Postgres;
use e2e_tests::program::Program;
use log::info;
use testcontainers_modules::testcontainers::runners::AsyncRunner;

#[allow(dead_code)]
/// Initial setup for e2e tests
struct Setup {
    pub postgres_instance: ContainerAsync<Postgres>,
    pub envs: Vec<(String, String)>,
}

use testcontainers_modules::testcontainers::ContainerAsync;

impl Setup {
    /// Initialise a new setup
    pub async fn new() -> Self {
        // Set up a postgres database port for testing
        let postgres_instance = Postgres::default().start().await.unwrap();

        let dataserver_endpoint = format!(
            "postgres://postgres:postgres@{}:{}/postgres",
            postgres_instance.get_host().await.unwrap(),
            postgres_instance.get_host_port_ipv4(5432).await.unwrap()
        );
        info!(
            "✅ PostgresDB setup completed with URL: {}",
            &dataserver_endpoint
        );

        Self {
            postgres_instance,
            envs: vec![
                ("WORKER__SCHEMA".to_string(), "worker_schema".to_string()),
                ("PG__URL".to_string(), dataserver_endpoint),
                ("PG__MAX_SIZE".to_string(), "10".to_string()),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use reqwest::Client;
    use serde_json::{json, Value};
    use test_log::test;
    use tokio_postgres::NoTls;
    use uuid::Uuid;

    use super::*;

    #[test(tokio::test)]
    async fn test_full_flow() {
        let setup_config = Setup::new().await;

        // setup irelia server
        let mut server_envs = setup_config.envs.clone();
        server_envs.append(&mut vec![
            ("SERVICE_NAME".to_string(), "irelia-server".to_string()),
            (
                "EXPORTER_ENDPOINT".to_string(),
                "127.0.0.1:7281".to_string(),
            ),
        ]);
        let (_, pg_url) = server_envs
            .iter()
            .find(|(key, _)| key == "PG__URL")
            .unwrap();
        let mut server = Program::run("SERVER".to_string(), "irelia", server_envs.clone());
        server.wait_till_started().await;
        let server_endpoint = format!("http://{}:{}", server.url, server.port);

        // setup irelia worker
        let mut worker_envs = setup_config.envs;
        worker_envs.append(&mut vec![
            ("SERVICE_NAME".to_string(), "irelia-worker".to_string()),
            ("WORKER__CONCURRENT".to_string(), "4".to_string()),
            (
                "EXPORTER_ENDPOINT".to_string(),
                "127.0.0.1:7281".to_string(),
            ),
        ]);
        let mut worker = Program::run("WORKER".to_string(), "irelia_worker", worker_envs);
        worker.wait_till_started().await;

        let client = Client::new();
        // test add job
        let cairo_pie =
            fs::read_to_string("./e2e-tests/assets/test_data/encoded_cairo_pie.txt").unwrap();
        test_add_job_incorrect_layout(client.clone(), server_endpoint.clone(), cairo_pie.clone())
            .await;
        println!("✅ test_add_job_incorrect_layout completed");

        test_add_job_additional_bad_flag(
            client.clone(),
            server_endpoint.clone(),
            cairo_pie.clone(),
        )
        .await;
        println!("✅ test_add_job_additional_bad_flag completed");

        test_add_job_no_cairo_job_id(client.clone(), server_endpoint.clone(), cairo_pie.clone())
            .await;
        println!("✅ test_add_job_no_cairo_job_id completed");

        test_add_job_incorrect_offchain_proof(
            client.clone(),
            server_endpoint.clone(),
            cairo_pie.clone(),
        )
        .await;
        println!("✅ test_add_job_incorrect_offchain_proof completed");

        test_add_job_successfully(client.clone(), server_endpoint.clone(), cairo_pie).await;
        println!("✅ test_add_job_successfully completed");

        // test get status
        // Set up the database
        setup_database(pg_url).await;
        println!("✅ Database setup completed");

        test_get_status_failed(client.clone(), server_endpoint.clone()).await;
        println!("✅ test_get_status_failed completed");

        test_get_status_invalid(client.clone(), server_endpoint.clone()).await;
        println!("✅ test_get_status_invalid completed");

        test_get_status_unknown(client.clone(), server_endpoint.clone()).await;
        println!("✅ test_get_status_unknown completed");

        test_get_status_in_progress(client.clone(), server_endpoint.clone()).await;
        println!("✅ test_get_status_in_progress completed");

        test_get_status_additional_bad_flag(client.clone(), server_endpoint.clone()).await;
        println!("✅ test_get_status_additional_bad_flag completed");

        test_get_status_not_created(client.clone(), server_endpoint.clone()).await;
        println!("✅ test_get_status_not_created completed");

        test_get_status_processed(client.clone(), server_endpoint.clone()).await;
        println!("✅ test_get_status_processed completed");

        test_get_status_onchain(client, server_endpoint).await;
        println!("✅ test_get_status_onchain completed");
    }

    //test add job function
    async fn test_add_job_incorrect_layout(
        client: Client,
        server_endpoint: String,
        cairo_pie: String,
    ) {
        let url =
            format!(
                "{}/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
                server_endpoint, Uuid::new_v4(), Uuid::new_v4(), true, "stark"
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

    async fn test_add_job_additional_bad_flag(
        client: Client,
        server_endpoint: String,
        cairo_pie: String,
    ) {
        let url = format!(
            "{}/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}&bla={}",
            server_endpoint, Uuid::new_v4(), Uuid::new_v4(), true, "starknet", true
        );
        let correct_body = cairo_pie.to_string();
        let expected = json!(
            {"code" : "JOB_RECEIVED_SUCCESSFULLY"}
        );
        let res = post_request(client, url, correct_body).await;
        assert_eq!(res, expected, "Response did not match expected value");
    }

    async fn test_add_job_no_cairo_job_id(
        client: Client,
        server_endpoint: String,
        cairo_pie: String,
    ) {
        let url = format!(
            "{}/v1/gateway/add_job?customer_id={}&offchain_proof={}&proof_layout={}",
            server_endpoint,
            Uuid::new_v4(),
            true,
            "starknet"
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

    async fn test_add_job_incorrect_offchain_proof(
        client: Client,
        server_endpoint: String,
        cairo_pie: String,
    ) {
        let url =
            format!(
                "{}/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
                server_endpoint, Uuid::new_v4(), Uuid::new_v4(), false, "starknet"
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

    async fn test_add_job_successfully(client: Client, server_endpoint: String, cairo_pie: String) {
        let url =
            format!(
                "{}/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
                server_endpoint, Uuid::new_v4(), Uuid::new_v4(), true, "starknet"
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

    // test get status function
    async fn test_get_status_failed(client: Client, server_endpoint: String) {
        let customer_id = "93bc3373-5115-4f99-aecc-1bc57997bfd3".to_string();
        let cairo_job_key = "11395dd2-b874-4c11-8744-ba6482da997d".to_string();

        let expected = json!(
            {
                "status" : "FAILED",
                "invalid_reason" : "",
                "error_log": "Sharp task failed",
                "validation_done": false
            }
        );
        let res = get_response(client, server_endpoint, customer_id, cairo_job_key).await;
        assert_eq!(res, expected, "Response did not match expected value");
    }

    async fn test_get_status_invalid(client: Client, server_endpoint: String) {
        let customer_id = "18dc4b30-8b46-42d1-8b51-aba8c8abc7b0".to_string();
        let cairo_job_key = "09a10775-7294-4e5d-abbc-7659caa1a330".to_string();

        let expected = json!(
            {
                "status" : "INVALID",
                "invalid_reason": "INVALID_CAIRO_PIE_FILE_FORMAT",
                "error_log": "The Cairo PIE file has a wrong format. \
                            Deserialization ended with \
                            exception: Invalid prefix for zip file..",
                "validation_done": false
            }
        );
        let res = get_response(client, server_endpoint, customer_id, cairo_job_key).await;
        assert_eq!(res, expected, "Response did not match expected value");
    }

    async fn test_get_status_unknown(client: Client, server_endpoint: String) {
        let customer_id = "2dd71442-58ca-4c35-a6de-8e637ff3c24b".to_string();
        let cairo_job_key = "f946ec7d-c3bf-42df-8bf0-9bcc751a8b3e".to_string();

        let expected = json!(
            {
                "status" : "UNKNOWN",
                "invalid_reason" : "",
                "error_log": "",
                "validation_done": false
            }
        );
        let res = get_response(client, server_endpoint, customer_id, cairo_job_key).await;
        assert_eq!(res, expected, "Response did not match expected value");
    }

    async fn test_get_status_in_progress(client: Client, server_endpoint: String) {
        let customer_id = "e703be2c-9ffe-4992-b968-da75da75d0b8".to_string();
        let cairo_job_key = "37e9d193-8e94-4df3-893a-cafa62a418c0".to_string();

        let expected = json!(
            {
                "status" : "IN_PROGRESS",
                "invalid_reason" : "",
                "error_log": "",
                "validation_done": false
            }
        );
        let res = get_response(client, server_endpoint, customer_id, cairo_job_key).await;
        assert_eq!(res, expected, "Response did not match expected value");
    }

    async fn test_get_status_additional_bad_flag(client: Client, server_endpoint: String) {
        let customer_id = "0581368e-2a32-4e93-b211-3f0ac9bae790".to_string();
        let cairo_job_key = "b01d3ad5-10db-4fcd-8746-fdc886de50bc".to_string();

        let expected = json!(
            {
                "status" : "IN_PROGRESS",
                "invalid_reason" : "",
                "error_log": "",
                "validation_done": true
            }
        );
        let res = get_response(client, server_endpoint, customer_id, cairo_job_key).await;
        assert_eq!(res, expected, "Response did not match expected value");
    }

    async fn test_get_status_not_created(client: Client, server_endpoint: String) {
        let customer_id = "040832f8-245f-4f05-a165-e2810e30047f".to_string();
        let cairo_job_key = "803eac13-3dbb-4ad2-a1df-311cfc2829cf".to_string();

        let expected = json!(
            {
                "status" : "NOT_CREATED",
                "invalid_reason" : "",
                "error_log": "",
                "validation_done": false
            }
        );
        let res = get_response(client, server_endpoint, customer_id, cairo_job_key).await;
        assert_eq!(res, expected, "Response did not match expected value");
    }

    async fn test_get_status_processed(client: Client, server_endpoint: String) {
        let customer_id = "8758d917-bbdc-4573-97ae-817e94fa31fb".to_string();
        let cairo_job_key = "59732e57-5722-4eb7-98db-8b90b89276f8".to_string();

        let expected = json!(
            {
                "status" : "PROCESSED",
                "invalid_reason" : "",
                "error_log": "",
                "validation_done": false
            }
        );
        let res = get_response(client, server_endpoint, customer_id, cairo_job_key).await;
        assert_eq!(res, expected, "Response did not match expected value");
    }

    async fn test_get_status_onchain(client: Client, server_endpoint: String) {
        let customer_id = "e3133ecb-e6e9-493a-ad64-ab9a4495af57".to_string();
        let cairo_job_key = "39af2c49-0c81-450e-91a9-aeff8dba2318".to_string();

        let expected = json!(
            {
                "status" : "ONCHAIN",
                "invalid_reason" : "",
                "error_log": "",
                "validation_done": true
            }
        );
        let res = get_response(client, server_endpoint, customer_id, cairo_job_key).await;
        assert_eq!(res, expected, "Response did not match expected value");
    }

    async fn get_response(
        client: Client,
        server_endpoint: String,
        customer_id: String,
        cairo_job_key: String,
    ) -> Value {
        let url = format!(
            "{}/v1/gateway/get_status?customer_id={}&cairo_job_key={}",
            server_endpoint, customer_id, cairo_job_key
        );
        client
            .get(&url)
            .send()
            .await
            .expect("Failed to send GET request")
            .json::<Value>()
            .await
            .expect("Failed to parse response body as JSON")
    }

    async fn setup_database(url: &str) {
        let (client, connection) = tokio_postgres::connect(url, NoTls)
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
        INSERT INTO jobs (id, customer_id, cairo_job_key, status, invalid_reason, error_log, validation_done)
        VALUES
            ('2a3ee88d-e19d-43ed-a79e-da9a28dc9525', '93bc3373-5115-4f99-aecc-1bc57997bfd3', '11395dd2-b874-4c11-8744-ba6482da997d','FAILED', '', 'Sharp task failed', false),

            ('58f667ea-67b3-4b32-b4f8-ef24ea1c8f12', '18dc4b30-8b46-42d1-8b51-aba8c8abc7b0', '09a10775-7294-4e5d-abbc-7659caa1a330', 'INVALID', 'INVALID_CAIRO_PIE_FILE_FORMAT', 'The Cairo PIE file has a wrong format. Deserialization ended with exception: Invalid prefix for zip file..', false),

            ('f2c604b7-52c5-4b69-9a67-de1276f9b8f8', '2dd71442-58ca-4c35-a6de-8e637ff3c24b', 'f946ec7d-c3bf-42df-8bf0-9bcc751a8b3e', 'UNKNOWN', '', '', false),

            ('d7045419-2b0f-4210-9e3d-7fb002839202', 'e703be2c-9ffe-4992-b968-da75da75d0b8', '37e9d193-8e94-4df3-893a-cafa62a418c0', 'IN_PROGRESS', '', '', false),

            ('18ef16cd-4511-4f29-a1d8-cd117d801f77', '0581368e-2a32-4e93-b211-3f0ac9bae790', 'b01d3ad5-10db-4fcd-8746-fdc886de50bc', 'IN_PROGRESS', '', '', true),

            ('549139a0-b288-401c-afb4-0f1018fd99f8', '040832f8-245f-4f05-a165-e2810e30047f', '803eac13-3dbb-4ad2-a1df-311cfc2829cf', 'NOT_CREATED', '', '', false),

            ('2283042d-f102-4ee6-a92f-73f3a86850e8', '8758d917-bbdc-4573-97ae-817e94fa31fb', '59732e57-5722-4eb7-98db-8b90b89276f8', 'PROCESSED', '', '', false),

            ('69f7ae7a-e981-44d2-9eb2-dfa551474870', 'e3133ecb-e6e9-493a-ad64-ab9a4495af57', '39af2c49-0c81-450e-91a9-aeff8dba2318', 'ONCHAIN', '', '', true);
    "#;

        client
            .batch_execute(reset_queries)
            .await
            .expect("Failed to reset database");
    }
}
