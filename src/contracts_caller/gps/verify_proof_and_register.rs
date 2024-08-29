use std::fs;
use std::str::FromStr;

use aptos_sdk::move_types::value::MoveValue;
use aptos_sdk::rest_client::aptos_api_types::MoveType;
use log::debug;

use crate::config::AppConfig;
use crate::contracts_caller::gps::types::verify_proof_and_register::{
    VerifyProofAndRegisterData, VerifyProofAndRegisterDataJson,
};
use crate::contracts_caller::transaction_helper::{get_event_from_transaction, send_tx};

pub async fn verify_proof_and_register(
    config: &AppConfig,
    data: &VerifyProofAndRegisterData,
) -> anyhow::Result<()> {
    let module_name = "gps_statement_verifier";
    debug!("pre_registered_facts");
    send_tx(
        config,
        "fact_registry",
        "register_facts",
        &vec![MoveValue::Vector(
            data.pre_registered_facts
                .iter()
                .map(|v| MoveValue::U256(*v))
                .collect(),
        )],
    )
    .await?;
    debug!("prepush_task_metadata");
    send_tx(
        config,
        module_name,
        "prepush_task_metadata",
        &vec![MoveValue::Vector(
            data.task_metadata
                .iter()
                .map(|v| MoveValue::U256(*v))
                .collect(),
        )],
    )
    .await?;
    debug!("prepush_data_to_verify_proof_and_register");
    send_tx(
        config,
        module_name,
        "prepush_data_to_verify_proof_and_register",
        &vec![
            MoveValue::Vector(
                data.proof_params
                    .iter()
                    .map(|v| MoveValue::U256(*v))
                    .collect(),
            ),
            MoveValue::Vector(data.proof.iter().map(|v| MoveValue::U256(*v)).collect()),
            MoveValue::Vector(
                data.cairo_aux_input
                    .iter()
                    .map(|v| MoveValue::U256(*v))
                    .collect(),
            ),
            MoveValue::U256(data.cairo_verifier_id),
        ],
    )
    .await?;
    let mut cnt_loop = 0;
    loop {
        cnt_loop += 1;
        debug!("verify_proof_and_register {}", cnt_loop);
        let tx = send_tx(config, module_name, "verify_proof_and_register", &vec![]).await?;
        let event = get_event_from_transaction(
            &tx,
            MoveType::from_str(&format!(
                "{}::{}::VparFinished",
                config.module_address, module_name
            ))?,
        );
        if event.is_ok() {
            break;
        }
    }
    Ok(())
}
