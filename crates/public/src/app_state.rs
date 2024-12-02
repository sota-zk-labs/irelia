use std::sync::Arc;

use crate::services::job::JobService;
use crate::services::worker_job::WorkerJobService;

#[derive(Clone)]
pub struct AppState {
    pub worker_service: Arc<WorkerJobService>,
    pub job_service: Arc<JobService>,
}

impl AppState {
    pub fn new(worker_service: Arc<WorkerJobService>, job_service: Arc<JobService>) -> Self {
        Self {
            worker_service,
            job_service,
        }
    }
}
