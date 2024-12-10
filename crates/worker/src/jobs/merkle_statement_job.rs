use graphile_worker::{IntoTaskHandlerResult, TaskHandler, WorkerContext};
use irelia_adapter::aptos_writer::contracts_caller::verify_merkle::extract_verify_merkle_input::extract_verify_merkle_input;
use irelia_adapter::aptos_writer::contracts_caller::verify_merkle::verify_merkle::verify_merkle;
use irelia_common::workers::{Worker, MERKLE_STATEMENT_VERIFIER_IDENTIFIER};
use irelia_core::entities::payload_verify_job::PayloadVerifyJob;
use serde::{Deserialize, Serialize};
use tracing::{info, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::app_state::State;

#[derive(Deserialize, Serialize, Debug)]
pub struct MerkleStatementJob(Worker<PayloadVerifyJob>);

impl TaskHandler for MerkleStatementJob {
    const IDENTIFIER: &'static str = MERKLE_STATEMENT_VERIFIER_IDENTIFIER;
    async fn run(self, ctx: WorkerContext) -> impl IntoTaskHandlerResult {
        let span = Span::current();
        let parent_cx = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&self.0.tracing)
        });

        span.set_parent(parent_cx);

        let state = ctx.extensions().get::<State>().unwrap();
        let verify_merkle_inputs =
            extract_verify_merkle_input(&self.0.data.sharp_proof.merkle_proofs.clone()).unwrap();
        info!("VERIFY MERKLE STATEMENT IN PROGRESS");
        for merkle_verify_input in verify_merkle_inputs {
            info!(
                "Verify merkle proof with expected root: {:?}",
                merkle_verify_input.expected_root
            );
            verify_merkle(&state.app_config, merkle_verify_input)
                .await
                .unwrap();
        }

        info!("VERIFY MERKLE STATEMENT SUCCESS");

        state.worker_port.verify_fri(self.0.data).await.unwrap();
    }
}
