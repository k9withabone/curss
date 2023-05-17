#![warn(clippy::pedantic)]

mod api;

use std::{net::SocketAddr, path::PathBuf};

use axum::{Router, Server};
use tower_http::services::ServeDir;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub address: SocketAddr,
    pub client_dir: PathBuf,
}

/// Starts a curss server with the given config
///
/// # Panics
/// Panics if binding to the given address fails
pub async fn run(config: Config) {
    let app = Router::new()
        .nest("/api", api::router())
        .fallback_service(ServeDir::new(config.client_dir));

    println!("Listening on {}", &config.address);
    Server::bind(&config.address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
