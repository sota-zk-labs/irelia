use std::io;

use irelia_core::common::core_error::CoreError;
use thiserror::Error;

// The kinds of errors we can hit in our application.
#[derive(Error, Debug)]
pub enum WorkerError {
    #[error("core error")]
    CoreError(#[from] CoreError),
    #[error("io error")]
    IOError(#[from] io::Error),
}
