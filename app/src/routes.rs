use askama_axum::IntoResponse;
use axum::extract::WebSocketUpgrade;
use axum_client_ip::InsecureClientIp;

use crate::{templates::{IndexTemplate, NotFoundTemplate}, websockets::handle_socket};

pub async fn root() -> IndexTemplate {
    IndexTemplate
}

pub async fn not_found() -> NotFoundTemplate {
    NotFoundTemplate
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    InsecureClientIp(ip): InsecureClientIp,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, ip.to_string()))
}
