use std::sync::Arc;
use std::time::Duration;

use clap::{Parser, Subcommand};
use deadpool_diesel::postgres::Pool;
use deadpool_diesel::{Manager, Runtime};
use irelia::app_state::AppState;
use irelia::options::Options;
use irelia::router::routes;
use irelia_adapter::worker::WorkerAdapter;
use irelia_common::cli_args::CliArgs;
use irelia_common::kill_signals;
use irelia_common::loggers::telemetry::init_telemetry;
use irelia_core::ports::worker::WorkerPort;
use opentelemetry::global;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use irelia_adapter::repositories::postgres::job_db::JobDBRepository;

#[tokio::main]
async fn main() {
    let options: Options = CliArgs::default_run_or_get_options(env!("APP_VERSION"));

    init_telemetry(
        options.service_name.as_str(),
        options.exporter_endpoint.as_str(),
        options.log.level.as_str(),
    );

    let server = tokio::spawn(serve(options));

    // Wait for the server to finish shutting down
    tokio::try_join!(server).expect("Failed to run server");

    global::shutdown_tracer_provider();
    info!("Shutdown successfully!");
}

/// Irelia Rest API.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Config file
    #[arg(short, long, default_value = "config/00-default.toml")]
    config_path: Vec<String>,
    /// Print version
    #[clap(short, long)]
    version: bool,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    /// Print config
    Config,
}

pub async fn serve(options: Options) {
    info!("Using postgres database: {}", &options.pg.url);
    let manager = Manager::new(&options.pg.url, Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .max_size(options.pg.max_size)
        .build()
        .unwrap();

    // TODO: use the same DB pool for the worker_adapter

    let job_repository = Arc::new(JobDBRepository::new(pool.clone()));


    let worker_adapter: Arc<dyn WorkerPort + Send + Sync> = Arc::new(
        WorkerAdapter::new(
            &options.pg.url,
            options.pg.max_size as u32,
            options.worker.schema.clone(),
        )
        .await,
    );
    let routes = routes(AppState::new(worker_adapter, job_repository, pool)).layer((
        TraceLayer::new_for_http(),
        // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
        // requests don't hang forever.
        TimeoutLayer::new(Duration::from_secs(10)),
    ));

    let endpoint = format!("{}:{}", options.server.url.as_str(), options.server.port);
    let listener = tokio::net::TcpListener::bind(endpoint.clone())
        .await
        .unwrap();
    info!("listening on http://{}", endpoint);
    axum::serve(listener, routes)
        .with_graceful_shutdown(kill_signals::wait_for_kill_signals())
        .await
        .unwrap();
}
