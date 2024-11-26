use graphile_worker::{IntoTaskHandlerResult, TaskHandler, WorkerContext};
use irelia_adapter::aptos_writer::contracts_caller::gps::extract_gps_input::extract_gps_input;
use irelia_adapter::aptos_writer::contracts_caller::gps::verify_proof_and_register::verify_proof_and_register;
use irelia_common::workers::{Worker, VERIFY_PROOF_AND_REGISTER_IDENTIFIER};
use irelia_core::entities::job::CairoJobStatus;
use irelia_core::entities::payload_verify_job::PayloadVerifyJob;
use serde::{Deserialize, Serialize};
use tracing::log::info;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::state::State;

#[derive(Deserialize, Serialize, Debug)]
pub struct VerifyAndRegisterJob(Worker<PayloadVerifyJob>);

impl TaskHandler for VerifyAndRegisterJob {
    const IDENTIFIER: &'static str = VERIFY_PROOF_AND_REGISTER_IDENTIFIER;
    async fn run(self, ctx: WorkerContext) -> impl IntoTaskHandlerResult {
        let span = Span::current();
        let parent_cx = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&self.0.tracing)
        });

        span.set_parent(parent_cx);
        let gps_verify_input = extract_gps_input(&self.0.data.sharp_proof.main_proof).unwrap();

        info!("VERIFY FRI IN PROGRESS");
        let state = ctx.extensions().get::<State>().unwrap();
        verify_proof_and_register(&state.app_config, &gps_verify_input)
            .await
            .unwrap();

        let mut job_status = self.0.data.job;
        job_status.status = CairoJobStatus::ONCHAIN;
        job_status.validation_done = true;

        state.job_port.update(job_status).await.unwrap();

        info!("=== VERIFY ONCHAIN SUCCESS ===");
    }
}
