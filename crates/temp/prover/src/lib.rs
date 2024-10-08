mod errors;

use std::path::PathBuf;

use adapter::annotated_proof::AnnotatedProof;
use scopeguard::defer;
use stone_cli::args::Network::ethereum;
use stone_cli::args::{LayoutName, SerializeArgs, VerifyArgs};
use stone_cli::bootloader::run_bootloader;
use stone_cli::prover::run_stone_prover_bootloader;
use stone_cli::serialize::serialize_proof;
use stone_cli::utils::{cleanup_tmp_files, parse, set_env_vars};
use stone_cli::verifier::run_stone_verifier;
use tempfile::Builder;

use crate::errors::GenerateProofError;
use crate::errors::GenerateProofError::{
    BootloaderError, JsonValueError, SerializationError, StoneProverError, TempDirError,
    UnsupportedLayoutError, VerifierError,
};

const CONFIG: &str = include_str!("../configs/env.json");
const PARAMETER_PATH: &str = "./configs/bootloader_cpu_air_params.json";
const BOOTLOADER_PROOF_NAME: &str = "bootloader_proof.json";
const FACT_TOPOLOGIES_PATH: &str = "fact_topologies.json";
const ANNOTATION_PATH: &str = "annotation.json";
const EXTRA_OUTPUT_PATH: &str = "extra_output.json";
const SERIALIZED_PROOF_PATH: &str = "bootloader_serialized_proof.json";

/// This code is adapted from: https://github.com/zksecurity/stone-cli/blob/main/src/main.rs
/// Generate proof from cairo pies or cairo programs
pub fn generate_proof(
    cairo_pies: Option<Vec<PathBuf>>,
    layout: LayoutName,
) -> Result<(serde_json::Value, AnnotatedProof), GenerateProofError> {
    if layout != LayoutName::starknet {
        return Err(UnsupportedLayoutError);
    }

    // load config file
    let config = parse(CONFIG);
    set_env_vars(&config);

    // make a temp folder for storing proof
    let proof_tmp_dir = Builder::new()
        .prefix("stone-cli-proof")
        .tempdir()
        .map_err(|_| TempDirError)?;

    defer! {
        cleanup_tmp_files(&proof_tmp_dir);
    }

    let tmp_dir = Builder::new()
        .prefix("stone-cli-")
        .tempdir()
        .map_err(|_| TempDirError)?;

    defer! {
        cleanup_tmp_files(&tmp_dir);
    }

    let parameter_file = Some(PathBuf::from(format!(
        "{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        PARAMETER_PATH
    )));

    // proof generator arguments
    let proof_args = stone_cli::args::ProveBootloaderArgs {
        cairo_programs,
        cairo_pies,
        layout,
        prover_config_file: Default::default(),
        parameter_file,
        output: PathBuf::from(proof_tmp_dir.path().join(BOOTLOADER_PROOF_NAME)),
        fact_topologies_output: PathBuf::from(proof_tmp_dir.path().join(FACT_TOPOLOGIES_PATH)),
        parameter_config: Default::default(),
        prover_config: Default::default(),
    };

    // generate bootloader proof
    run_bootloader(&proof_args, &tmp_dir)
        .map_err(|e| BootloaderError(e.to_string()))
        .and_then(|run_bootloader_result| {
            run_stone_prover_bootloader(
                &proof_args,
                &run_bootloader_result.air_public_input,
                &run_bootloader_result.air_private_input,
                &tmp_dir,
            )
            .map_err(|e| StoneProverError(e.to_string()))
        })?;

    // verify proof
    let verify_args = VerifyArgs {
        proof: PathBuf::from(proof_tmp_dir.path().join(BOOTLOADER_PROOF_NAME)),
        annotation_file: Some(PathBuf::from(proof_tmp_dir.path().join(ANNOTATION_PATH))),
        extra_output_file: Some(PathBuf::from(proof_tmp_dir.path().join(EXTRA_OUTPUT_PATH))),
    };
    run_stone_verifier(verify_args).map_err(|e| VerifierError(e.to_string()))?;

    // serialize proof
    let serialize_args = SerializeArgs {
        proof: PathBuf::from(proof_tmp_dir.path().join(BOOTLOADER_PROOF_NAME)),
        network: ethereum,
        output: PathBuf::from(proof_tmp_dir.path().join(SERIALIZED_PROOF_PATH)),
        annotation_file: Some(PathBuf::from(proof_tmp_dir.path().join(ANNOTATION_PATH))),
        extra_output_file: Some(PathBuf::from(proof_tmp_dir.path().join(EXTRA_OUTPUT_PATH))),
    };
    serialize_proof(serialize_args).map_err(|e| SerializationError(e.to_string()))?;

    let origin_proof_file =
        std::fs::read_to_string(proof_tmp_dir.path().join(SERIALIZED_PROOF_PATH)).unwrap();
    let annotated_proof: AnnotatedProof =
        serde_json::from_str(&origin_proof_file).map_err(|_| JsonValueError)?;

    let topologies_file =
        std::fs::read_to_string(proof_tmp_dir.path().join(FACT_TOPOLOGIES_PATH)).unwrap();
    let topology_json: serde_json::Value =
        serde_json::from_str(&topologies_file).map_err(|_| JsonValueError)?;
    Ok((topology_json, annotated_proof))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_proof() {
        let cairo_programs = None;
        let cairo_pie = Some(vec![PathBuf::from("./tests/fibonacci_with_output.zip")]);
        let layout = LayoutName::starknet;
        let res = generate_proof(cairo_programs, cairo_pie, layout);
        assert!(res.is_ok());
        let (topology_json, annotated_proof) = res.unwrap();
    }
}
