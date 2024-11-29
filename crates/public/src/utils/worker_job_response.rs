use irelia_core::entities::worker_job::WorkerJobStatus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WorkerJobResponse {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub fn successfully() -> WorkerJobResponse {
    WorkerJobResponse {
        code: "JOB_RECEIVED_SUCCESSFULLY".to_string(),
        message: None,
    }
}

pub fn internal_server_error() -> WorkerJobResponse {
    WorkerJobResponse {
        code: "500".to_string(),
        message: Some("Internal server error".to_string()),
    }
}

pub fn get_worker_job_response(code: WorkerJobStatus) -> WorkerJobResponse {
    match code {
        WorkerJobStatus::FaultyCairoPie => successfully(),
        WorkerJobStatus::IncorrectLayout => internal_server_error(),
        WorkerJobStatus::AdditionalBadFlag => successfully(),
        WorkerJobStatus::NoCairoJobId => internal_server_error(),
        WorkerJobStatus::IncorrectOffchainProof => internal_server_error(),

        WorkerJobStatus::Successfully => successfully(),
    }
}

#[cfg(test)]
mod test {
    use crate::utils::worker_job_response::{get_worker_job_response, WorkerJobResponse};
    use irelia_core::entities::worker_job::WorkerJobStatus;

    #[test]
    fn test_get_worker_job_response() {
        let success = WorkerJobResponse {
            code: "JOB_RECEIVED_SUCCESSFULLY".to_string(),
            message: None,
        };

        let error = WorkerJobResponse {
            code: "500".to_string(),
            message: Some("Internal server error".to_string()),
        };

        let test_faulty_cairo_pie = get_worker_job_response(WorkerJobStatus::FaultyCairoPie);
        assert_eq!(success, test_faulty_cairo_pie);

        let test_incorrect_layout = get_worker_job_response(WorkerJobStatus::IncorrectLayout);
        assert_eq!(error, test_incorrect_layout);

        let test_additional_bad_flag = get_worker_job_response(WorkerJobStatus::AdditionalBadFlag);
        assert_eq!(success, test_additional_bad_flag);

        let test_no_cairo_job_id = get_worker_job_response(WorkerJobStatus::NoCairoJobId);
        assert_eq!(error, test_no_cairo_job_id);

        let test_incorrect_offchain_proof =
            get_worker_job_response(WorkerJobStatus::IncorrectOffchainProof);
        assert_eq!(error, test_incorrect_offchain_proof);

        let test_successfully = get_worker_job_response(WorkerJobStatus::Successfully);
        assert_eq!(success, test_faulty_cairo_pie);
    }
}
