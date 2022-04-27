use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;

use std::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: i32,
    pub auth_token: String,
    pub database: Database,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        // Get environment name from system environment variable, default is development
        let env_name = env::var("ENV").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("config/{}", env_name)).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

lazy_static! {
    pub static ref CONFIG: RwLock<AppConfig> = RwLock::new(AppConfig::new().unwrap());
}
