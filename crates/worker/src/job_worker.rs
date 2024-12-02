use std::path::PathBuf;
use std::time::Duration;

use graphile_worker::{IntoTaskHandlerResult, TaskHandler, WorkerContext};
use irelia_adapter::prover::stone_prover::StoneProver;
use irelia_adapter::repositories::postgres::job_db::JobDBRepository;
use irelia_common::workers::{Worker, ADD_JOB_WORKER_IDENTIFIER};
use irelia_core::entities::job::JobStatus::InProgress;
use irelia_core::entities::job::{JobEntity, JobId};
use irelia_core::entities::worker_job::WorkerJobEntity;
use irelia_core::ports::prover::ProverPort;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{info, instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::state::State;

#[derive(Deserialize, Serialize, Debug)]
pub struct JobWorker(Worker<WorkerJobEntity>);

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
        // let state = _ctx.extensions().get::<State>().unwrap();

        sleep(Duration::from_secs(5)).await;
        info!("data: {:?}", worker_job);
    }
}
