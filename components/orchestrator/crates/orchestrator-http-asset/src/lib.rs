//! Asset HTTP endpoints

mod handler;
mod handler_write;
mod routes;

pub use handler::{CreateAssetRequest, UpdateAssetRequest};
pub use routes::routes;
