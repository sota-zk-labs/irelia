use std::sync::Arc;

use rust_core::ports::question::QuestionPort;

/// Router for handling HTTP requests related to questions.
#[derive(Clone)]
pub struct AppState {
    pub question_port: Arc<dyn QuestionPort + Send + Sync + 'static>,
}

impl AppState {
    /// Creates a new Router instance with the specified QuestionPort.
    pub fn new(question_port: Arc<dyn QuestionPort + Send + Sync + 'static>) -> Self {
        AppState {
            question_port: question_port.clone(),
        }
    }
}
