use std::fmt::Debug;
use std::process::exit;

use clap::{Parser, Subcommand};
use serde::Deserialize;

use crate::options::parse_options;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Config file
    #[arg(short, long, default_value = "config/00-default.toml")]
    config_path: Vec<String>,
    /// Print version
    #[clap(short, long)]
    version: bool,
}

impl CliArgs {
    pub fn default_run_or_get_options<'de, T: Deserialize<'de> + Debug>(version: &str) -> T {
        let args = Self::parse();
        if args.version {
            println!("{}", version);
            exit(0);
        }

        let options: T = match parse_options(args.config_path) {
            Ok(options) => options,
            Err(err) => {
                println!("Failed to load config: {}", err);
                exit(0);
            }
        };

        if let Some(Commands::Config) = args.command {
            println!("{:#?}", &options);
            exit(0);
        }

        options
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Print config
    Config,
}