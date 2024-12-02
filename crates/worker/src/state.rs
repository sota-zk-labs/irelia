use std::sync::Arc;

use irelia_core::ports::job::JobPort;

#[derive(Clone, Debug)]
pub struct State {
    pub job_port: Arc<dyn JobPort + Send + Sync>,
}

impl State {
    pub fn new(job_port: Arc<dyn JobPort + Send + Sync>) -> Self {
        State { job_port }
    }
}
