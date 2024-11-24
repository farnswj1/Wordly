use std::time::Duration;

use axum::{body::Body, http::{Request, Response}};
use axum_client_ip::InsecureClientIp;
use tower_http::trace::MakeSpan;
use tracing::{info, span, Level, Span};

#[derive(Debug, Clone)]
pub struct IPSpan;

impl<B> MakeSpan<B> for IPSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let InsecureClientIp(ip) = InsecureClientIp::from(request.headers(), request.extensions()).unwrap();
        span!(
            Level::INFO,
            "request",
            ip = %ip,
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version()
        )
    }
}

pub fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    let status = response.status().as_u16();
    info!("status={status} latency={latency:?}");
}
