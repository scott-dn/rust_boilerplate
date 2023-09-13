use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// config path location
    #[arg(short = 'c', long, default_value_t = format!("{}/.{}/config.toml", env!("HOME"), env!("CARGO_PKG_NAME")))]
    pub config: String,

    /// log format
    #[arg(short='f', long, value_enum, default_value_t=LogFmt::Json)]
    pub log_format: LogFmt,

    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Clone, ValueEnum)]
pub enum LogFmt {
    Json,
    Txt,
}

#[derive(Args)]
pub struct ServerOpts {
    /// set env mode
    #[arg(long, value_enum, default_value_t = Mode::Local)]
    pub mode: Mode,

    /// set http port
    #[arg(long, default_value_t = 8080)]
    pub http_port: u32,
}

#[derive(Clone, ValueEnum)]
pub enum Mode {
    Local,
    Dev,
    Uat,
    Prod,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Start a server
    Start(ServerOpts),
}
