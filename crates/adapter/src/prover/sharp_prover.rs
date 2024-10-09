use rust_core::common::prover_error::ProverError;
use rust_core::common::prover_error::ProverError::{JsonValueError, UnsupportedLayoutError};
use rust_core::entities::annotated_proof::{AnnotatedProof, SplitProofs};
use rust_core::entities::sharp_proof::SharpProof;

use crate::prover::annotation_parser::split_fri_merkle_statements;
use crate::prover::default_prime;
use crate::prover::oods_statement::{
    main_proof_to_json, memory_page_registration_args, FactTopology,
};

pub fn generate_sharp_proof(
    topology_json: serde_json::Value,
    annotated_proof: AnnotatedProof,
    layout: usize,
) -> Result<SharpProof, ProverError> {
    if layout == 6 {
        generate_layout6_proof(topology_json, annotated_proof)
    } else {
        Err(UnsupportedLayoutError)
    }
}

fn generate_layout6_proof(
    topology_json: serde_json::Value,
    annotated_proof: AnnotatedProof,
) -> Result<SharpProof, ProverError> {
    let mut merkle_proofs: Vec<String> = vec![];
    let mut fri_proofs: Vec<String> = vec![];
    let mut memory_pages: Vec<String> = vec![];
    let split_proofs: SplitProofs = split_fri_merkle_statements(annotated_proof.clone())
        .map_err(|_| ProverError::ParseError)?;
    let fact_topologies: Vec<FactTopology> =
        serde_json::from_value(topology_json.get("fact_topologies").unwrap().clone())
            .map_err(|_| JsonValueError)?;

    for fri_statement in split_proofs.fri_merkle_statements {
        fri_proofs.push(fri_statement.to_json());
    }

    for i in 0..split_proofs.merkle_statements.len() {
        let key = format!("Trace {}", i);
        let trace_merkle = split_proofs.merkle_statements.get(&key).unwrap();
        merkle_proofs.push(trace_merkle.to_json());
    }

    let (_, continuous_pages) = memory_page_registration_args(&split_proofs.main_proof);
    for page in continuous_pages {
        let page_json = page.to_json(
            split_proofs.main_proof.interaction_z,
            split_proofs.main_proof.interaction_alpha,
            default_prime(),
        );
        memory_pages.push(page_json);
    }

    let main_proof = main_proof_to_json(&split_proofs.main_proof, fact_topologies, 6);
    Ok(SharpProof {
        merkle_proofs,
        fri_proofs,
        memory_pages,
        main_proof,
        layout: 6,
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_proof() -> Result<(), Box<dyn std::error::Error>> {
        let origin_proof_file = include_str!("./test_samples/bootloader_serialized_proof.json");
        let annotated_proof: AnnotatedProof = serde_json::from_str(&origin_proof_file)?;

        let topologies_file = include_str!("./test_samples/fact_topologies.json");
        let topology_json: serde_json::Value = serde_json::from_str(&topologies_file).unwrap();

        let proof = generate_sharp_proof(topology_json, annotated_proof, 6).unwrap();

        assert_eq!(proof.fri_proofs.len(), 8);
        assert_eq!(proof.merkle_proofs.len(), 3);
        assert_eq!(proof.memory_pages.len(), 1);
        assert_eq!(proof.main_proof.len(), 46740);
        Ok(())
    }
}
