//! Asset HTTP endpoints

mod handler;
mod routes;

pub use handler::{CreateAssetRequest, UpdateAssetRequest};
pub use routes::routes;
