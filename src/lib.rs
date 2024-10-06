mod errors;

use std::path::PathBuf;

use adapter::proof::Proof;
use prover::generate_proof;
use stone_cli::args::LayoutName;

use crate::errors::FlowError;
use crate::errors::FlowError::{AdapterError, ProverError, UnsupportedLayoutError};

pub fn full_flow(
    cairo_programs: Option<Vec<PathBuf>>,
    cairo_pies: Option<Vec<PathBuf>>,
    layout: LayoutName,
) -> Result<(), FlowError> {
    if layout != LayoutName::starknet {
        return Err(UnsupportedLayoutError);
    }
    let (topology_json, annotated_proof) = generate_proof(cairo_programs, cairo_pies, layout)
        .map_err(|e| ProverError(e.to_string()))?;

    let proof =
        Proof::new(topology_json, annotated_proof, 6).map_err(|e| AdapterError(e.to_string()))?;

    Ok(())
}
