use std::{env, net::SocketAddr};

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: SocketAddr,
    pub database_url: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let app_port = env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());
        let addr: SocketAddr = format!("{app_host}:{app_port}")
            .parse()
            .context("APP_HOST or APP_PORT is invalid")?;

        let database_url = env::var("DATABASE_URL").context("DATABASE_URL is not set")?;

        Ok(Self { addr, database_url })
    }
}
