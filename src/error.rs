#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error("io error {0}")]
    IOError(#[from] std::io::Error),

    #[error("transaction not succeed")]
    TransactionNotSucceed,

    #[error("not found")]
    NotFound,
}