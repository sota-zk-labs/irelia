use std::path::PathBuf;
use async_trait::async_trait;
use stone_cli::args::LayoutName;

#[async_trait]
pub trait ProverPort {
    async fn generate_proof(&self,
                            layout: LayoutName,
                            cairo_pies: Option<Vec<PathBuf>>
    ) -> Result<(serde_json::Value, AnnotatedProof), GenerateProofError>;
}
