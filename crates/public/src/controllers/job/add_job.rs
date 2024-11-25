use std::fmt::Display;
use std::str;

use crate::app_state::AppState;
use crate::json_response::JsonResponse;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use irelia_core::entities::job::{JobEntity, JobId, JobResponse};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;
use crate::utils::save_cairo_pie;

// request and response: https://docs.google.com/document/d/1-9ggQoYmjqAtLBGNNR2Z5eLreBmlckGYjbVl0khtpU0/edit?tab=t.0
#[instrument(level = "info", skip(app_state))]
pub async fn add_job(
    State(app_state): State<AppState>,
    Query(params): Query<AddJobQueryParams>,
    encoded_cairo_pie: String,
) -> Result<JsonResponse<JobResponse>, StatusCode> {
    // Todo: Check params.customer_id

    // Off chain flag set to proof
    if !params.offchain_proof {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mut job_entity = JobEntity {
        id: JobId(Uuid::new_v4()),
        customer_id: params.customer_id,
        cairo_job_key: params.cairo_job_key,
        offchain_proof: params.offchain_proof,
        proof_layout: params.proof_layout.to_string(),
        cairo_pie: "".to_string(),
        // status: Default::default(),
        // invalid_reason: None,
        // error_log: None,
        // validation_done: None,
    };
    let mut response = None;

    // Todo: Add more field to JobEntity
    // Sending a job with a faulty cairo pie
    if let Ok(cairo_pie) = save_cairo_pie(&encoded_cairo_pie, &job_entity.id.0.to_string()) {
        job_entity.cairo_pie = cairo_pie.to_str().unwrap().to_string();
    } else {
        // job_entity.status = "INVALID".to_string();
        // job_entity.invalid_reason = Some("INVALID_CAIRO_PIE_FILE_FORMAT".to_string());
        // job_entity.error_log = Some(format!(
        //     "The Cairfo PIE file has a wrong format. Deserialization ended with exception: {}",
        //     decoded_pie.err().unwrap().to_string()
        // ));
        response = Some(JobResponse {
            code: Some("JOB_RECEIVED_SUCCESSFULLY".to_string()),
            message: None,
        });
    }

    let job_entity = app_state.worker_port.add(job_entity).await.unwrap();

    let response = response.unwrap_or(JobResponse {
        code: Some("JOB_RECEIVED_SUCCESSFULLY".to_string()),
        message: Some(job_entity.id.0.to_string()),
    });
    Ok(JsonResponse(response))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddJobQueryParams {
    customer_id: String,
    cairo_job_key: String,
    offchain_proof: bool,
    proof_layout: ProofLayout,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ProofLayout {
    Small,
}

impl Display for ProofLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProofLayout::Small => "small".to_string(),
            }
        )
    }
}

#[cfg(test)]
mod test {
    use reqwest::Client;
    use serde_json::json;

    #[tokio::test]
    async fn test_add_job() {
        let client = Client::new();
        let url = "http://127.0.0.1:8000/v1/gateway/add_job";
        let cairo_pie = "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";
        // Sending a job with a faulty cairo pie
        let res = client
            .post(url)
            .query(&json!({
                "customer_id": "",
                "cairo_job_key": "String",
                "offchain_proof": true,
                "proof_layout": "small",
            }))
            .body(format!("{}x", cairo_pie))
            .send()
            .await
            .unwrap();
        assert!(res.text().await.unwrap().contains("JOB_RECEIVED_SUCCESSFULLY"));
    }
}
