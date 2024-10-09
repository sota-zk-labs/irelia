use async_trait::async_trait;

use crate::common::prover_error::ProverError;
use crate::entities::sharp_proof::SharpProof;
#[async_trait]
pub trait ProverPort {
    async fn generate_proof(&self) -> Result<SharpProof, ProverError>;
}
