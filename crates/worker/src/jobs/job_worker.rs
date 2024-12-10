use std::path::PathBuf;

use graphile_worker::{IntoTaskHandlerResult, TaskHandler, WorkerContext};
use irelia_adapter::prover::stone_prover::StoneProver;
use irelia_common::workers::{Worker, ADD_JOB_WORKER_IDENTIFIER};
use irelia_core::entities::payload_verify_job::PayloadVerifyJob;
use irelia_core::entities::worker_job::WorkerJobEntity;
use irelia_core::ports::prover::ProverPort;
use serde::{Deserialize, Serialize};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::app_state::State;

#[derive(Deserialize, Serialize, Debug)]
pub struct JobWorker(Worker<WorkerJobEntity>);

impl TaskHandler for JobWorker {
    const IDENTIFIER: &'static str = ADD_JOB_WORKER_IDENTIFIER;

    async fn run(self, ctx: WorkerContext) -> impl IntoTaskHandlerResult {
        let span = Span::current();
        let parent_cx = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&self.0.tracing)
        });

        span.set_parent(parent_cx);

        let state = ctx.extensions().get::<State>().unwrap();

        let job = state
            .job_port
            .get_job(self.0.data.customer_id, self.0.data.cairo_job_key)
            .await
            .unwrap();

        let stone_prover = StoneProver {
            layout: self.0.data.proof_layout,
            cairo_pie: vec![PathBuf::from(self.0.data.cairo_pie)],
        };
        let proof = stone_prover.generate_proof().await.unwrap();
        let payload = PayloadVerifyJob {
            job,
            sharp_proof: proof,
        };

        state.worker_port.verify_merkle(payload).await.unwrap();
    }
}
