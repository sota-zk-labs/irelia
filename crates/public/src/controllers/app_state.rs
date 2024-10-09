use std::sync::Arc;

use rust_core::ports::job::JobPort;

/// Router for handling HTTP requests related to questions.
#[derive(Clone)]
pub struct AppState {
    pub job_port: Arc<dyn JobPort + Send + Sync + 'static>,
}

impl AppState {
    /// Creates a new Router instance with the specified QuestionPort.
    pub fn new(prover_port: Arc<dyn JobPort + Send + Sync + 'static>) -> Self {
        AppState {
            job_port: prover_port.clone(),
        }
    }
}
