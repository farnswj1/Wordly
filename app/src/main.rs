mod config;
mod data;
mod log;
mod routes;
mod templates;
mod websockets;

use std::net::SocketAddr;

use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::http::HeaderValue;
use axum::{http::Method, routing::get, serve, Router};
use axum_client_ip::SecureClientIpSource;
use config::Config;
use dotenvy::dotenv;
use log::{on_response, IPSpan};
use routes::{not_found, root, ws_handler};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnFailure, TraceLayer};
use tracing::{info, Level};

fn get_router() -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    let config = envy::from_env::<Config>().unwrap();

    let origins = config.cors_allowed_origins
        .split(" ")
        .map(|origin| origin.parse().unwrap())
        .collect::<Vec<HeaderValue>>();

    // Set up CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET]);

    // Set up logging middleware
    let logger = TraceLayer::new_for_http()
        .make_span_with(IPSpan)
        .on_response(on_response)
        .on_failure(DefaultOnFailure::new().level(Level::INFO));

    // Enable serving static files
    let serve_dir = ServeDir::new("static");

    Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler))
        .nest_service("/static", serve_dir)
        .fallback(not_found)
        .layer(SecureClientIpSource::RightmostXForwardedFor.into_extension())
        .layer(logger)
        .layer(cors)
        .into_make_service_with_connect_info::<SocketAddr>()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let address = "0.0.0.0:8000";
    let router = get_router();
    let listener = TcpListener::bind(address).await.unwrap();

    info!("LISTENING on {address}");
    serve(listener, router).await.unwrap();
}
