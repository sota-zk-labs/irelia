use std::num::ParseIntError;

use aptos_sdk::move_types::u256::U256FromStrError;

#[derive(thiserror::Error, Debug)]
pub enum AptosWriterError {
    #[error("io error {0}")]
    IOError(#[from] std::io::Error),

    #[error("parse int error {0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("parse u256 error {0}")]
    ParseU256Error(#[from] U256FromStrError),

    #[error("parse vm status error {0}")]
    ParseVmStatusError(String),

    #[error("transaction not succeed {0}")]
    TransactionNotSucceed(String),

    #[error("flow not finished {0}")]
    FlowNotFinished(String),

    #[error("not found")]
    NotFound,

    #[error("property not found")]
    PropertyNotFound,
}
