//! Scene HTTP endpoints

mod handler;
mod routes;

pub use handler::{CreateSceneRequest, UpdateSceneRequest};
pub use routes::routes;
