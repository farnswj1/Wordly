use std::net::SocketAddr;

use askama_axum::IntoResponse;
use axum::extract::{ConnectInfo, WebSocketUpgrade};

use crate::{templates::{IndexTemplate, NotFoundTemplate}, websockets::handle_socket};

pub async fn root() -> IndexTemplate {
    IndexTemplate
}

pub async fn not_found() -> NotFoundTemplate {
    NotFoundTemplate
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}
