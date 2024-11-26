use std::collections::HashMap;
use std::str::FromStr;

use async_trait::async_trait;
use graphile_worker::WorkerUtils;
use irelia_common::workers::{
    Worker, ADD_JOB_WORKER_IDENTIFIER, MERKLE_STATEMENT_VERIFIER_IDENTIFIER,
    REGISTER_CONTINUOUS_IDENTIFIER, VERIFY_FRI_IDENTIFIER, VERIFY_PROOF_AND_REGISTER_IDENTIFIER,
};
use irelia_core::common::core_error::CoreError;
use irelia_core::entities::payload_verify_job::PayloadVerifyJob;
use irelia_core::entities::worker_job::WorkerJobEntity;
use irelia_core::ports::worker::WorkerPort;
use sqlx::postgres::PgConnectOptions;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub struct WorkerAdapter {
    pub worker_utils: WorkerUtils,
}

impl WorkerAdapter {
    pub async fn new(url: &str, max_connections: u32, schema: String) -> Self {
        let pg_options = PgConnectOptions::from_str(url).unwrap();
        let pg_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(max_connections)
            .connect_with(pg_options)
            .await
            .unwrap();

        let worker_utils = WorkerUtils::new(pg_pool, schema);
        Self { worker_utils }
    }
}

#[async_trait]
impl WorkerPort for WorkerAdapter {
    async fn add(&self, job: WorkerJobEntity) -> Result<WorkerJobEntity, CoreError> {
        // retrieve the current span
        let span = tracing::Span::current();
        // retrieve the current context
        let cx = span.context();
        // inject the current context through the amqp headers
        let mut tracing_info = HashMap::new();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&cx, &mut tracing_info)
        });
        let payload = Worker {
            data: &job,
            tracing: tracing_info,
        };
        self.worker_utils
            .add_raw_job(ADD_JOB_WORKER_IDENTIFIER, payload, Default::default())
            .await
            .unwrap();
        Ok(job)
    }

    async fn verify_merkle(&self, job: PayloadVerifyJob) -> Result<(), CoreError> {
        // retrieve the current span
        let span = tracing::Span::current();
        // retrieve the current context
        let cx = span.context();
        // inject the current context through the amqp headers
        let mut tracing_info = HashMap::new();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&cx, &mut tracing_info)
        });
        let payload = Worker {
            data: &job,
            tracing: tracing_info,
        };
        self.worker_utils
            .add_raw_job(
                MERKLE_STATEMENT_VERIFIER_IDENTIFIER,
                payload,
                Default::default(),
            )
            .await
            .unwrap();
        Ok(())
    }

    async fn verify_fri(&self, job: PayloadVerifyJob) -> Result<(), CoreError> {
        // retrieve the current span
        let span = tracing::Span::current();
        // retrieve the current context
        let cx = span.context();
        // inject the current context through the amqp headers
        let mut tracing_info = HashMap::new();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&cx, &mut tracing_info)
        });
        let payload = Worker {
            data: &job,
            tracing: tracing_info,
        };
        self.worker_utils
            .add_raw_job(VERIFY_FRI_IDENTIFIER, payload, Default::default())
            .await
            .unwrap();
        Ok(())
    }

    async fn register_memory_page(&self, job: PayloadVerifyJob) -> Result<(), CoreError> {
        // retrieve the current span
        let span = tracing::Span::current();
        // retrieve the current context
        let cx = span.context();
        // inject the current context through the amqp headers
        let mut tracing_info = HashMap::new();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&cx, &mut tracing_info)
        });
        let payload = Worker {
            data: &job,
            tracing: tracing_info,
        };
        self.worker_utils
            .add_raw_job(REGISTER_CONTINUOUS_IDENTIFIER, payload, Default::default())
            .await
            .unwrap();
        Ok(())
    }

    async fn verify_proof_and_register(&self, job: PayloadVerifyJob) -> Result<(), CoreError> {
        // retrieve the current span
        let span = tracing::Span::current();
        // retrieve the current context
        let cx = span.context();
        // inject the current context through the amqp headers
        let mut tracing_info = HashMap::new();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&cx, &mut tracing_info)
        });
        let payload = Worker {
            data: &job,
            tracing: tracing_info,
        };
        self.worker_utils
            .add_raw_job(
                VERIFY_PROOF_AND_REGISTER_IDENTIFIER,
                payload,
                Default::default(),
            )
            .await
            .unwrap();
        Ok(())
    }
}
