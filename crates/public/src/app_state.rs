use std::sync::Arc;

use deadpool_diesel::postgres::Pool;
use irelia_core::ports::job::JobPort;
use irelia_core::ports::worker::WorkerPort;

#[derive(Clone)]
pub struct AppState {
    pub job_port: Arc<dyn JobPort + Send + Sync>,
    pub worker_port: Arc<dyn WorkerPort + Send + Sync>,
    pub db: Pool,
}

impl AppState {
    pub fn new(
        job_port: Arc<dyn JobPort + Send + Sync>,
        worker_port: Arc<dyn WorkerPort + Send + Sync>,
        db: Pool,
    ) -> Self {
        Self {
            job_port,
            worker_port,
            db,
        }
    }
}
