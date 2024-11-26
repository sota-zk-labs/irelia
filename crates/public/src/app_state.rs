use std::sync::Arc;

use deadpool_diesel::postgres::Pool;
use irelia_core::ports::job::JobPort;
use irelia_core::ports::worker_job::WorkerJobPort;
use irelia_core::ports::worker::WorkerPort;



#[derive(Clone)]
pub struct AppState {
    pub worker_job_port: Arc<dyn WorkerJobPort + Send + Sync>,
    pub worker_port: Arc<dyn WorkerPort + Send + Sync>,
    pub db: Pool,

    pub job_port: Arc<dyn JobPort + Send + Sync>
}

impl AppState {
    pub fn new(
        worker_job_port: Arc<dyn WorkerJobPort + Send + Sync>,
        worker_port: Arc<dyn WorkerPort + Send + Sync>,
        db: Pool,

        job_port: Arc<dyn JobPort + Send + Sync>
    ) -> Self {
        Self {
            worker_job_port,
            worker_port,
            db,

            job_port
        }
    }
}
