use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenerateProofError {
    #[error("unsupported layout error")]
    UnsupportedLayoutError,
    #[error("temporary directory error")]
    TempDirError,
    #[error("bootloader error")]
    BootloaderError(String),
    #[error("stone temp error")]
    StoneProverError(String),
    #[error("verifier error")]
    VerifierError(String),
    #[error("serialization error")]
    SerializationError(String),
    #[error("json value error")]
    JsonValueError,
}
