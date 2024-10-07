#[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use openssl;
#[rustfmt::skip]
#[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use diesel;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use adapter::repositories::in_memory::question::QuestionInMemoryRepository;
use adapter::repositories::postgres::question_db::QuestionDBRepository;
use clap::{Parser, Subcommand};
use common::kill_signals;
use common::loggers::telemetry::init_telemetry;
use common::options::parse_options;
use deadpool_diesel::postgres::Pool;
use deadpool_diesel::{Manager, Runtime};
use irelia::controllers::app_state::AppState;
use irelia::options::Options;
use irelia::router::routes;
use opentelemetry::global;
use rust_core::ports::question::QuestionPort;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.version {
        println!(env!("APP_VERSION"));
        return;
    }

    let options: Options = match parse_options(args.config_path) {
        Ok(options) => options,
        Err(err) => {
            println!("Failed to load config: {}", err);
            return;
        }
    };

    if let Some(Commands::Config) = args.command {
        println!("{:#?}", options);
        return;
    }

    init_telemetry(
        options.service_name.as_str(),
        options.exporter_endpoint.as_str(),
        options.log.level.as_str(),
    );

    let (tx, rx) = oneshot::channel();
    let server = tokio::spawn(serve(options, rx));

    kill_signals::wait_for_kill_signals().await;

    // Send the shutdown signal
    let _ = tx.send(());

    // Wait for the server to finish shutting down
    tokio::try_join!(server).expect("Failed to run server");

    global::shutdown_tracer_provider();
    info!("Shutdown successfully!");
}

/// Simple REST server.
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

pub async fn serve(options: Options, rx: Receiver<()>) {
    let question_port: Arc<dyn QuestionPort + Send + Sync> = if options.db.in_memory.is_some() {
        info!("Using in-memory database");
        Arc::new(QuestionInMemoryRepository::new())
    } else if options.db.pg.is_some() {
        let database_config = options.db.pg.clone().unwrap();
        info!("Using postgres database: {}", database_config.url);
        let manager = Manager::new(database_config.url, Runtime::Tokio1);
        let pool = Pool::builder(manager)
            .max_size(database_config.max_size)
            .build()
            .unwrap();
        Arc::new(QuestionDBRepository::new(pool))
    } else {
        info!("No database specified, falling back to in-memory");
        Arc::new(QuestionInMemoryRepository::new())
    };

    let routes = routes(AppState { question_port }).layer((
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
