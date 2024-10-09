use std::str::FromStr;
use std::time::Duration;

use common::cli_args::CliArgs;
use common::kill_signals::wait_for_kill_signals;
use common::loggers::telemetry::init_telemetry;
use graphile_worker::{IntoTaskHandlerResult, TaskHandler, WorkerContext, WorkerOptions};
use irelia_worker::options::{DBConfig, Options, Server};
use irelia_worker::router::routes;
use irelia_worker::state::State;
use opentelemetry::global;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgConnectOptions;
use tokio::time::sleep;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    let options: Options = CliArgs::default_run_or_get_options(env!("APP_VERSION"));

    init_telemetry(
        options.service_name.as_str(),
        options.exporter_endpoint.as_str(),
        options.log.level.as_str(),
    );

    let server = tokio::spawn(serve_api(options.server.clone()));

    run_workers(options.pg.clone()).await;

    // Wait for the server to finish shutting down
    tokio::try_join!(server).expect("Failed to run server");

    global::shutdown_tracer_provider();
    info!("Shutdown successfully!");
}

#[derive(Deserialize, Serialize)]
struct ShowRunCount;

impl TaskHandler for ShowRunCount {
    const IDENTIFIER: &'static str = "show_run_count";

    async fn run(self, ctx: WorkerContext) -> impl IntoTaskHandlerResult {
        sleep(Duration::from_secs(5)).await;
        info!("Run count: 1");
        Err("asdsd")
    }
}

async fn serve_api(server_opt: Server) {
    let routes = routes().layer((
        TraceLayer::new_for_http(),
        // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
        // requests don't hang forever.
        TimeoutLayer::new(Duration::from_secs(10)),
    ));

    let endpoint = format!("{}:{}", server_opt.url.as_str(), server_opt.port);
    let listener = tokio::net::TcpListener::bind(endpoint.clone())
        .await
        .unwrap();
    info!("Listening on http://{}", endpoint);
    axum::serve(listener, routes)
        .with_graceful_shutdown(wait_for_kill_signals())
        .await
        .unwrap();
}

pub async fn run_workers(pg: DBConfig) {
    info!("Using postgres database: {}", &pg.url);
    let pg_options = PgConnectOptions::from_str(&pg.url).unwrap();

    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(pg.max_size)
        .connect_with(pg_options)
        .await
        .unwrap();

    let worker = WorkerOptions::default()
        .concurrency(3)
        .schema("example_simple_worker")
        .define_job::<ShowRunCount>()
        .pg_pool(pg_pool)
        .add_extension(State::new())
        .init()
        .await
        .unwrap();

    let utils = worker.create_utils();

    for _ in 0..10 {
        utils
            .add_job(ShowRunCount, Default::default())
            .await
            .unwrap();
    }
    worker.run().await.unwrap();
}
