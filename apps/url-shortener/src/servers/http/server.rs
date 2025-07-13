use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
pub async fn run_http_server(host: &str, port: u16) {
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    let public_routes = Router::new().route("/url", get(|| async { "Hello, world! url" }));
    let app = axum::Router::new()
        .nest("/api", public_routes)
        .layer(get_cors());

    axum::serve(listener, app).await.unwrap();
}
fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
