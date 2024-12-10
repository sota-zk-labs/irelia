use graphile_worker::{IntoTaskHandlerResult, TaskHandler, WorkerContext};
use irelia_adapter::aptos_writer::contracts_caller::verify_fri::extract_verify_fri_input::extract_verify_fri_input;
use irelia_adapter::aptos_writer::contracts_caller::verify_fri::verify_fri::verify_fri;
use irelia_common::workers::{Worker, VERIFY_FRI_IDENTIFIER};
use irelia_core::entities::payload_verify_job::PayloadVerifyJob;
use serde::{Deserialize, Serialize};
use tracing::log::info;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::app_state::State;

#[derive(Deserialize, Serialize, Debug)]
pub struct VerifyFriJob(Worker<PayloadVerifyJob>);

impl TaskHandler for VerifyFriJob {
    const IDENTIFIER: &'static str = VERIFY_FRI_IDENTIFIER;
    async fn run(self, ctx: WorkerContext) -> impl IntoTaskHandlerResult {
        let span = Span::current();
        let parent_cx = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&self.0.tracing)
        });

        span.set_parent(parent_cx);
        let fri_verify_inputs =
            extract_verify_fri_input(&self.0.data.sharp_proof.fri_proofs.clone()).unwrap();

        info!("VERIFY FRI IN PROGRESS");
        let state = ctx.extensions().get::<State>().unwrap();

        for fri_verify_input in fri_verify_inputs {
            let (
                fri_verify_input,
                proof,
                fri_queue,
                evaluation_point,
                fri_step_size,
                expected_root,
            ) = fri_verify_input;
            verify_fri(
                &state.app_config,
                fri_verify_input,
                proof,
                fri_queue,
                evaluation_point,
                fri_step_size,
                expected_root.clone(),
            )
            .await
            .unwrap();
            info!("Verify FRI success with expected root {:?}", expected_root);
        }

        state
            .worker_port
            .register_memory_page(self.0.data)
            .await
            .unwrap();
    }
}
