use deadpool_postgres::{Manager, Pool};
use dotenv::dotenv;
use serde::Deserialize;
use std::convert::TryInto;
use tokio::stream::StreamExt;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new().separator("__"))?;
        cfg.try_into()
    }
}
