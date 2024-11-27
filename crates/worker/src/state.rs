use std::sync::Arc;
use deadpool_diesel::postgres::Pool;
use deadpool_diesel::{Manager, Runtime};
use tracing::info;
use irelia_adapter::repositories::postgres::job_db::JobDBRepository;
use irelia_common::cli_args::CliArgs;
use irelia_core::ports::job::JobPort;
use crate::options::Options;

#[derive(Clone)]
pub struct State {
    pub job_port: Arc<dyn JobPort + Send + Sync>,
}

impl State {
    pub fn new() -> Self {
        let options: Options = CliArgs::default_run_or_get_options(env!("APP_VERSION"));
        info!("Using postgres database: {}", &options.pg.url);
        let manager = Manager::new(&options.pg.url, Runtime::Tokio1);
        let pool = Pool::builder(manager)
            .max_size(options.pg.max_size.try_into().unwrap())
            .build()
            .unwrap();

        let job_port = Arc::new(JobDBRepository::new(pool.clone()));

        State {
            job_port,
        }
    }
}
