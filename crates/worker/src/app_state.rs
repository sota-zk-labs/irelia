use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use irelia_adapter::aptos_writer::config::AppConfig;
use irelia_core::ports::job::JobPort;
use irelia_core::ports::worker::WorkerPort;

#[derive(Clone)]
pub struct State {
    pub job_port: Arc<dyn JobPort + Send + Sync>,
    pub worker_port: Arc<dyn WorkerPort + Send + Sync>,
    pub app_config: AppConfig,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").finish()
    }
}

impl State {
    pub fn new(
        job_port: Arc<dyn JobPort + Send + Sync>,
        worker_port: Arc<dyn WorkerPort + Send + Sync>,
        app_config: AppConfig,
    ) -> Self {
        State {
            job_port,
            worker_port,
            app_config,
        }
    }
}
