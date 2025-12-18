//! Health check routes

use warp::Filter;

use crate::handler::handle_health;

/// Health check routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "health")
        .and(warp::get())
        .and_then(handle_health)
}
