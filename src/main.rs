use clap::Parser;
use cli::{Cli, Cmd};
use tracing::{debug, info, trace};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    tracing_subscriber::registry()
        .with(fmt::layer().pretty())
        .with(EnvFilter::from_default_env())
        .init();

    info!("hello");
    debug!("hello");
    trace!("hello");
    match cli.command {
        Cmd::Start(_) => {}
    }
}
