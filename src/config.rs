use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub log_format: LogFmt,
    pub server: Server,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum LogFmt {
    Json,
    Txt,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Server {
    pub mode: Mode,
    pub http_port: u16,
    pub pg_url: String,
    pub pg_max_pool: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Mode {
    Local,
    Dev,
    Uat,
    Prod,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_format: LogFmt::Json,
            server: Server {
                mode: Mode::Local,
                http_port: 8080,
                pg_url: "postgres://service:password@localhost:5432/book?sslmode=disable".into(),
                pg_max_pool: 50,
            },
        }
    }
}

impl Config {
    pub fn from_file(file: PathBuf) -> Result<Self> {
        if !file.exists() {
            if let Some(parent_dir) = file.parent() {
                create_dir_all(parent_dir)?;
                let config = Config::default();
                let mut new_file = File::create(file)?;
                let toml = toml::to_string(&config)?;
                new_file.write_all(toml.as_bytes())?;
                return Ok(config);
            }
            bail!("parent dir not found");
        }

        let toml = read_to_string(file)?;
        Ok(toml::from_str(&toml)?)
    }
}
