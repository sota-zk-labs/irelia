use std::collections::HashMap;
use std::str;
use axum::body::{Body, Bytes};
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::Json;
use irelia_core::entities::worker_job::{WorkerJob, WorkerJobId};
use irelia_core::entities::job::{JobEntity, JobId, JobStatus};
use irelia_core::ports::job::JobPort;
use openssl::pkey::Params;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use tracing::log::info;
use uuid::Uuid;
use irelia_core::entities::job::JobStatus::InProgress;

use crate::app_state::AppState;
use crate::errors::AppError;
use crate::json_response::JsonResponse;
use crate::services::check::check_worker_job;
use crate::utils::worker_job_response::{get_worker_job_response, WorkerJobResponse};

#[derive(Debug, Deserialize)]
pub struct Request {
    cairo_pie: String,
}
#[derive(Debug, Deserialize)]
pub struct CairoPieReq {
    action: String,
    request: Request,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NewWorkerJob {
    pub customer_id: String,
    pub cairo_job_key: String,
    pub offchain_proof: bool,
    pub proof_layout: String,
    pub bla: Option<bool>,
}

#[instrument(level = "info", skip(app_state))]
pub async fn add_job(
    State(app_state): State<AppState>,
    Query(params): Query<NewWorkerJob>,
    Json(req): Json<CairoPieReq>
) -> Result<JsonResponse<WorkerJobResponse>, AppError> {
    info!("{:?}", params);
    let (response_code, is_valid) = check_worker_job(params.clone());

    if !is_valid  {
        return  Ok(JsonResponse(get_worker_job_response(response_code)))
    }

    let worker_job = app_state
        .worker_port
        .add(WorkerJob {
            id: WorkerJobId(Uuid::new_v4()),
            customer_id: params.customer_id,
            cairo_job_key: params.cairo_job_key,
            offchain_proof: params.offchain_proof,
            proof_layout: params.proof_layout,
            cairo_pie: req.request.cairo_pie,
        })
        .await?;

    // TODO: Process the data
    let _ = app_state
        .job_port
        .add(JobEntity {
            id: JobId(Uuid::new_v4()),
            customer_id: worker_job.customer_id.clone(),
            cairo_job_key: worker_job.cairo_job_key.clone(),
            status: InProgress,
            validation_done: false,
        })
        .await?;

    Ok(JsonResponse(get_worker_job_response(response_code)))
}

#[cfg(test)]
mod test {
    use reqwest::Client;
    use serde_json::json;
    use uuid::Uuid;
    use tokio;
    #[tokio::test]
    async fn test_add_job() {
        let client = Client::new();

        let correct_body = json!(
            {
                "action": "add_job",
                "request": {
                    "cairo_pie": "UEsDBBQAAAAIAAAAIQB4ytupXQEAAKMDAAANAAAAbWV0YWRhdGEuanNvbq2S24rCQAyGX0V67UXOk+yrLCJd7UphraIVZMV339QDylLvhKGkyTf\
                    //JPJqdruNqtdva4+JqeM23WTERu6AhV3M0NGQrYoTJlhCIUCCGrECIVQmVHDLCIRCMoKsnohIBDH6aRa1n2dqp8UwIIiEhii4RzTSdbFkKwgOkUwpjz4dKLoqWd\
                    sooQu7kwX+HU+EIHIAJkLCRpkPv9JtAiJkxIhJmzXE9mj5BdRFE3KRcHTkxqXkupeNOFhcRLgxYyxMApIKqDYoK3mwgYcpqn6cCEUyq6STl8IjJJKkJ0NESJwZna\
                    SF77e+EKv+vS+IwqPNmGWo/F1aH/6ttsP41FtDv320FdDfl23XebgnPFtQuf7ZrVuuv4yqW23bI4DkPV9+zsMLekAN8dmcejbTTeK4wM3HPBd08+/t6MsP1i4o9v\
                    FKCr/0Nut7uz+At9u97yPntyfr+77Xf287XN2/gNQSwMEFAAAAAgAAAAhAIbCeb3VAQAAwA0AAAoAAABtZW1vcnkuYmlujdXJVsJAEAXQJEAIYZ5xxgkV5xGnBUt\
                    /wa9J7934ux7FSMxLThZUXhUbFpeqV306xLL+y4RBGNimXLSEsuFsCaAcOMssAssU5pIrpC6a675JrqicW1L2c5X9ypl+lvHFfl7q7OWn2JZcBc4lc324MFgs93D\
                    mkqumcyM3EfPV4Bwytw73G0Q34WMsuQbcIr4vYtsmnHihUC24aOOfQD7ntjJfR9mvm7rvpZuKrpe6/HPuJ04o5PcGab/8+zKE+wpXF87NG8Gxc0mmxl/E2x8/F\
                    4mTn5L4PqX9xGtgCnB+brCkbOqK6Kf6P1A4V5mvDId9pDKerp+pKPP5cOz5rcJViKtlnZHz1eFG+e1MQ5mvCVcgrgXnEdeGw3wxXweO7dFV5uvBlYjrw5WJG8Bhv\
                    phvCMf2GCnzrcGxc16HY++tjWw+I+fbzDq5zJYy3zZcnbgdOHYuYzjMF/PtwrE99pT59uH6xB3AsffqIRzmi/kmcGyPI2W+4+zcnDqBY8/bFA7zxXyncGyPM2W+c\
                    7gZcRdw7P/vEg7zxXxXcGyPa2W+G7h34m7h2PvoDg7zxXz3cGyPB2W+GdwncY9whFlPcJgv5nuGY3u8KPPF52tT94p+TuZXq+b+AVBLAwQUAAAACAAAACEArCBY\
                    hC0AAAAzAAAAFAAAAGFkZGl0aW9uYWxfZGF0YS5qc29uq1bKLy0pKC2JTyrNzCnJzFOyUqhWKkhMTy0GsWp1FJQSS0qKMpNKS6AitbUAUEsDBBQAAAAIAAAAIQBh\
                    oo8TwAAAFcAAAAYAAAAZXhlY3V0aW9uX3Jlc291cmNlcy5qc29uq1bKiy8uSS0oVrJSMDfSUVBKKs3MKcnMi8/MKy5JzEtOjU/OL80rSS0Cylcr5ZeWFJSWxEPV\
                    AIWMaoFa8uJzU3PziyrjM/JzUkEGGdQCAFBLAwQUAAAACAAAACEA2YDFkhYAAAAUAAAADAAAAHZlcnNpb24uanNvbqtWSk7MLMqPL8hMVbJSUDLUM1SqBQBQSwE\
                    CFAMUAAAACAAAACEAeMrbqV0BAACjAwAADQAAAAAAAAAAAAAAgAEAAAAAbWV0YWRhdGEuanNvblBLAQIUAxQAAAAIAAAAIQCGwnm91QEAAMANAAAKAAAAAAAAAA\
                    AAAACAAYgBAABtZW1vcnkuYmluUEsBAhQDFAAAAAgAAAAhAKwgWIQtAAAAMwAAABQAAAAAAAAAAAAAAIABhQMAAGFkZGl0aW9uYWxfZGF0YS5qc29uUEsBAhQDF\
                    AAAAAgAAAAhAGFiijxPAAAAVwAAABgAAAAAAAAAAAAAAIAB5AMAAGV4ZWN1dGlvbl9yZXNvdXJjZXMuanNvblBLAQIUAxQAAAAIAAAAIQDZgMWSFgAAABQAAAAMA\
                    AAAAAAAAAAAAACAAWkEAAB2ZXJzaW9uLmpzb25QSwUGAAAAAAUABQA1AQAAqQQAAAAA"
                }
            }
        );

        let url_correct_worker_job = format!(
            "http://localhost:8000/v1/gateway/add_job?customer_id={}&cairo_job_key={}&offchain_proof={}&proof_layout={}",
            Uuid::new_v4().to_string(), Uuid::new_v4().to_string(), true, "small".to_string()
        );
        let res = client
            .post(&url_correct_worker_job)
            .json(&correct_body)
            .send()
            .await
            .expect("Failed to send POST request");

        // Parse and print the response body (if JSON)
        if let Ok(body) = res.json::<serde_json::Value>().await {
            println!("Response Body: {}", body);
        } else {
            println!("Failed to parse response body as JSON.");
        }
    }
}
