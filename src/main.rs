// src/main.rs

mod auth;
mod config;
mod db;
mod routes;
mod state;
mod tls;

use axum::{Router, Server};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use tls::tls_config;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Load .env config
    dotenv().ok();

    // Set up tracing/logging
    tracing_subscriber::fmt::init();

    // Load config and initialize app state
    let config = config::load();
    let state = state::build_app_state(&config).await;

    // Set up all routes with shared state
    let app = Router::new()
        .merge(routes::auth_routes::routes())
        .merge(routes::status::routes())
        .merge(routes::video::routes())
        .merge(routes::static_files::routes())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Bind to address and serve with TLS
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let tls_config = tls_config(&config).expect("Failed to load TLS config");

    tracing::info!("🚀 Starting SecureEdge on https://{}", addr);
    Server::bind(&addr)
        .serve(axum_server::tls_rustls::RustlsAcceptor::new(tls_config).bind(app))
        .await
        .expect("Server crashed");
}
