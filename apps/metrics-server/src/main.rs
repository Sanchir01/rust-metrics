mod app;
mod pkg;
mod server;
mod feature;
use dotenvy::dotenv;
use crate::pkg::db::clickhouse::connect_click_house;
use crate::app::config::Config;
use crate::server::grpc::server::MetricsGrpcServer;
use std::env;
#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc as GlobalAlloc;

#[cfg(target_os = "windows")]
use mimalloc::MiMalloc as GlobalAlloc;
use tonic::transport::Server;
use candles_proto::metrics::metrics_server::MetricsServer;

#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok().expect("Could not load .env file");
    let config = Config::new().await;
    println!("Config: {:?}", config);

    let db_cfg = match &config.clickhouse {
        Some(cfg) => cfg,
        None => {
            eprintln!("Database config not found");
            return Err("Database config not found".into());
        }
    };
    let clickhouse_password = env::var("CLICKHOUSE_PASSWORD")
        .expect("CLICKHOUSE_PASSWORD must be set");
    let client =    match connect_click_house(
        &db_cfg.url,
        &db_cfg.database,
        &clickhouse_password,
        &db_cfg.username,
    ).await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("❌ ClickHouse connection error: {:?}", err);
            return Err(err.into());
        }
    };
    let metric = MetricsGrpcServer::new();
    let addr = "0.0.0.0:50051".parse()?;
    println!("gRPC Server running at {}", addr);
    Server::builder().add_service(MetricsServer::new(metric)).serve(addr).await?;

    Ok(())
}
