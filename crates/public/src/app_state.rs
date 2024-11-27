use std::sync::Arc;

use deadpool_diesel::postgres::Pool;
use irelia_core::ports::job::JobPort;
use irelia_core::ports::worker::WorkerPort;



#[derive(Clone)]
pub struct AppState {
    pub worker_port: Arc<dyn WorkerPort + Send + Sync>,
    pub job_port: Arc<dyn JobPort + Send + Sync>,
    pub db: Pool
}

impl AppState {
    pub fn new(
        worker_port: Arc<dyn WorkerPort + Send + Sync>,
        job_port: Arc<dyn JobPort + Send + Sync>,
        db: Pool
    ) -> Self {
        Self {
            worker_port,
            job_port,
            db,
        }
    }
}
