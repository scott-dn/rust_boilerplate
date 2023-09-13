use std::path::PathBuf;

use clap::Parser;
use cli::{Cli, Cmd};
use config::{Config, LogFmt};
use tracing::{debug, info, trace};
use tracing_subscriber::{prelude::*, EnvFilter};

mod cli;
mod config;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = match Config::from_file(PathBuf::from(&cli.config)) {
        Ok(config) => config,
        Err(e) => panic!("couldn't read config file: {}\n{}", cli.config, e),
    };

    // init logger
    match config.log_format {
        LogFmt::Json => {
            tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer().json())
                .with(EnvFilter::from_default_env())
                .init();
        }
        LogFmt::Txt => {
            tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer().pretty())
                .with(EnvFilter::from_default_env())
                .init();
        }
    };

    info!("hello");
    debug!("hello");
    trace!("hello");
    match cli.command {
        Cmd::Start => {}
    }
}
