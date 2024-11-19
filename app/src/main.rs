mod config;
mod data;
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
use routes::{not_found, root, ws_handler};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;

fn get_router() -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    let config = envy::from_env::<Config>().unwrap();

    let origins = config.cors_allowed_origins
        .split(" ")
        .map(|origin| origin.parse().unwrap())
        .collect::<Vec<HeaderValue>>();

    // Set up middleware layers
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET]);

    // Enable serving static files
    let serve_dir = ServeDir::new("static");

    Router::new()
        .layer(SecureClientIpSource::RightmostXForwardedFor.into_extension())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .route("/", get(root))
        .route("/ws", get(ws_handler))
        .nest_service("/static", serve_dir)
        .fallback(not_found)
        .into_make_service_with_connect_info::<SocketAddr>()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let address = "0.0.0.0:8000";
    let router = get_router();
    let listener = TcpListener::bind(address).await.unwrap();

    info!("LISTENING on {address}");
    serve(listener, router).await.unwrap();
}
