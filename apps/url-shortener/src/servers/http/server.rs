use std::sync::Arc;

use crate::{app::handlers::Handlers, feature::url::handler::get_all_url_handler_axum};
use axum::{Router, extract::State, routing::get, Json};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};


pub async fn run_http_server(host: &str, port: u16, handlers: Arc<Handlers>) {
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    let public_routes = Router::new().route(
        "/url",
        get(get_all_url_handler_axum),
    ).with_state(handlers.url_handler.clone());
    let app = axum::Router::new()
        .nest("/api", public_routes)
        .with_state(handlers)
        .layer(get_cors());

    axum::serve(listener, app).await.unwrap();
}
fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
