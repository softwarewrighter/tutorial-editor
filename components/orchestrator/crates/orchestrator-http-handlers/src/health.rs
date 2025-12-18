use serde::Serialize;
use std::convert::Infallible;

pub async fn handle_health() -> Result<impl warp::Reply, Infallible> {
    #[derive(Serialize)]
    struct HealthReply {
        status: &'static str,
    }
    Ok(warp::reply::json(&HealthReply { status: "ok" }))
}
