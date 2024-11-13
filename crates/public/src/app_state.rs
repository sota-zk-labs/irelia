use std::sync::Arc;

use irelia_core::ports::job::JobPort;
use irelia_core::ports::worker::WorkerPort;

#[derive(Clone)]
pub struct AppState {
    pub job_port: Arc<dyn JobPort + Send + Sync>,
    pub worker_port: Arc<dyn WorkerPort + Send + Sync>,
}

impl AppState {
    pub fn new(
        job_port: Arc<dyn JobPort + Send + Sync>,
        worker_port: Arc<dyn WorkerPort + Send + Sync>,
    ) -> Self {
        Self {
            job_port,
            worker_port,
        }
    }
}
