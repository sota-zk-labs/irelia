use std::time::Duration;

use graphile_worker::{IntoTaskHandlerResult, TaskHandler, WorkerContext};
use irelia_common::workers::{Worker, ADD_JOB_WORKER_IDENTIFIER};
use irelia_core::entities::worker_job::WorkerJob;
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

        eprintln!("span = {:#?}", span);
        eprintln!("parent_cx = {:#?}", parent_cx);
        eprintln!("self = {:#?}", self);
        span.set_parent(parent_cx);

        sleep(Duration::from_secs(5)).await;
        info!("data: {:?}", self.0.data);
    }
}