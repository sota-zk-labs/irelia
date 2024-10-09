use crate::aptos_writer::contracts_caller::gps::types::verify_proof_and_register::{
    VerifyProofAndRegisterData, VerifyProofAndRegisterDataJson,
};

pub fn extract_gps_input(main_proof: &String) -> anyhow::Result<VerifyProofAndRegisterData> {
    let data = serde_json::from_str::<VerifyProofAndRegisterDataJson>(main_proof)?;
    Ok(VerifyProofAndRegisterData::from(data))
}
