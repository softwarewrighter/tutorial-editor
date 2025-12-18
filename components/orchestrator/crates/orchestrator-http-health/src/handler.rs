//! Health check handler

use serde::Serialize;
use std::convert::Infallible;

/// Health check response
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

/// Handle health check request
pub async fn handle_health() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&HealthResponse { status: "ok" }))
}
