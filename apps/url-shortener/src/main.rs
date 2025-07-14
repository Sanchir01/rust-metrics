use crate::app::config::Config;
use crate::servers::http::server::run_http_server;
use crate::utils::db::init_primary_db;

use dotenvy::dotenv;
mod app;
mod servers;
mod feature;
mod utils;
#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc as GlobalAlloc;

#[cfg(target_os = "windows")]
use mimalloc::MiMalloc as GlobalAlloc;

#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Could not load .env file");

    let config = Config::new().await;
    let http_server = config.server.clone().unwrap_or_else(|| {
        panic!("HTTP server configuration not found");
    });
    let _ = init_primary_db(&config).await.expect("Count not init db");
    run_http_server(&http_server.host, http_server.port).await;

    Ok(())
}
