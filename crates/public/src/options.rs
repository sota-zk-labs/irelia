use irelia_adapter::repositories::postgres::config::DBConfig;
use irelia_common::options::{default_log, Log};
use serde::Deserialize;

/// Configuration options for the application.
///
/// This struct represents the configuration options for the application, including server settings,
/// database configuration, endpoint for the exporter, service name, and logging configuration.
#[readonly::make]
#[derive(Deserialize, Debug)]
pub struct Options {
    /// Configuration for the server.
    pub server: Server,
    /// Specifies the configuration of database will be connected.
    pub pg: DBConfig,
    /// Configuration for the worker.
    pub worker: Worker,
    /// The endpoint for the exporter.
    pub exporter_endpoint: String,
    /// The name of the service.
    pub service_name: String,
    /// Configuration for logging, including log level.
    #[serde(default = "default_log")]
    pub log: Log,
}

/// Represents server configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    /// Port number for the server.
    pub port: u16,
    /// URL for the server.
    pub url: String,
}

/// Represents worker configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct Worker {
    /// The postgresql schema to use for the worker.
    pub schema: String,
}
