use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Worker<T> {
    pub data: T,
    pub tracing: HashMap<String, String>,
}

pub const ADD_JOB_WORKER_IDENTIFIER: &str = "add_job";
