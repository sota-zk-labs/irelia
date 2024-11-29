use std::path::PathBuf;
use std::time::Duration;

use crate::state::State;
use graphile_worker::{IntoTaskHandlerResult, TaskHandler, WorkerContext};
use irelia_adapter::prover::stone_prover::StoneProver;
use irelia_adapter::repositories::postgres::job_db::JobDBRepository;
use irelia_common::workers::{Worker, ADD_JOB_WORKER_IDENTIFIER};
use irelia_core::entities::job::JobStatus::InProgress;
use irelia_core::entities::job::{JobEntity, JobEntityPayload, JobId};
use irelia_core::entities::worker_job::WorkerJob;
use irelia_core::ports::prover::ProverPort;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{info, instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Deserialize, Serialize, Debug)]
pub struct JobWorker(Worker<WorkerJob>);

impl TaskHandler for JobWorker {
    const IDENTIFIER: &'static str = ADD_JOB_WORKER_IDENTIFIER;

    #[instrument(level = "info", skip(self, _ctx))]
    async fn run(self, _ctx: WorkerContext) -> impl IntoTaskHandlerResult {
        let span = Span::current();
        let parent_cx = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&self.0.tracing)
        });

        span.set_parent(parent_cx);

        sleep(Duration::from_secs(10)).await;

        ///TODO: Processing Data
        //Set processing
        let worker_job = self.0.data;

        // let _ = State::new()
        //     .job_port
        //     .update(JobEntityPayload {
        //         customer_id: worker_job.clone().customer_id,
        //         cairo_job_key: worker_job.clone().cairo_job_key,
        //         status: InProgress,
        //         validation_done: false,
        //     })
        //     .await
        //     .unwrap();

        // Process data

        // let cairo_pie = vec![PathBuf::from(
        //     "/home/andrew/workspace/irelia/crates/adapter/src/prover/test_samples/fibonacci_with_output.zip",
        // )];
        //
        // let layout_name = worker_job.proof_layout;
        // let stone_prover = StoneProver {
        //     cairo_pie,
        //     layout: layout_name.parse().unwrap(),
        // };
        // let proof = stone_prover.generate_proof().await.unwrap();

        sleep(Duration::from_secs(5)).await;
        info!("data: {:?}", worker_job);
    }
}
