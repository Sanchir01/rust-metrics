use serde::{Deserialize, Serialize};
use tokio::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config{
    pub clickhouse: Option<ClickhouseConfig>,
    pub grpc_server: Option<GRPCServerConfig>
}
#[derive(Debug, Deserialize,Serialize)]
pub struct ClickhouseConfig {
    pub url: String,
    pub password: String,
    pub username: String,
    pub database: String,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct GRPCServerConfig {
    pub host: String,
    pub port: u16,
}


impl Config {
    pub async fn new() -> Self {
        let config_path = if cfg!(debug_assertions) {
            println!("[DEV MODE]");
            "apps/metrics-server/src/config/dev.toml"
        } else {
            println!("[PROD MODE]");
            "apps/metrics-server/src/config/prod.toml"
        };

        match fs::read_to_string(config_path).await {
            Ok(content) => toml::from_str(&content).unwrap_or_else(|err| {
                eprintln!("Error parsing config: {:?}", err);
                Self::default()
            }),
            Err(err) => {
                eprintln!("Error reading config file: {:?}", err);
                Self::default()
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clickhouse: None,
            grpc_server: None,
        }
    }
}