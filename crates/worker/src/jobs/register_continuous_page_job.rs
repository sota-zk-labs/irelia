use graphile_worker::{IntoTaskHandlerResult, TaskHandler, WorkerContext};
use irelia_adapter::aptos_writer::contracts_caller::memory_page_fact_registry::extract_register_memory::extract_register_continuous_page;
use irelia_adapter::aptos_writer::contracts_caller::memory_page_fact_registry::register_continuous_memory_page::register_continuous_memory_page;
use irelia_common::workers::{Worker, REGISTER_CONTINUOUS_IDENTIFIER};
use irelia_core::common::core_error::CoreError;
use irelia_core::entities::payload_verify_job::PayloadVerifyJob;
use serde::{Deserialize, Serialize};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::app_state::State;

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterContinuousJob(Worker<PayloadVerifyJob>);

impl TaskHandler for RegisterContinuousJob {
    const IDENTIFIER: &'static str = REGISTER_CONTINUOUS_IDENTIFIER;
    async fn run(self, ctx: WorkerContext) -> impl IntoTaskHandlerResult {
        let span = Span::current();
        let parent_cx = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&self.0.tracing)
        });

        span.set_parent(parent_cx);
        let register_continuous_page_inputs =
            extract_register_continuous_page(&self.0.data.sharp_proof.memory_pages)?;

        let state = ctx.extensions().get::<State>().unwrap();

        for page in register_continuous_page_inputs {
            register_continuous_memory_page(&state.app_config, page).await?;
        }

        state
            .worker_port
            .verify_proof_and_register(self.0.data)
            .await?;
        Ok::<(), CoreError>(())
    }
}
