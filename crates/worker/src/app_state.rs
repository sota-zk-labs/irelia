use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use irelia_core::ports::job::JobPort;

#[derive(Clone)]
pub struct State {
    pub job_port: Arc<dyn JobPort + Send + Sync>,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").finish()
    }
}

impl State {
    pub fn new(job_port: Arc<dyn JobPort + Send + Sync>) -> Self {
        State { job_port }
    }
}
