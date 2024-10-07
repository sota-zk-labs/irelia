use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlowError {
    #[error("unsupported layout error")]
    UnsupportedLayoutError,
    #[error("prover error")]
    ProverError(String),
    #[error("adapter error")]
    AdapterError(String),
    #[error("verify error")]
    VerifyError(String),
}
