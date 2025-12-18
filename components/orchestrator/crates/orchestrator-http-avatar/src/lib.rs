//! Avatar HTTP endpoints

mod handler;
mod handler_pipeline;
mod routes;

pub use handler::{GenerateAudioRequest, GenerateVideoRequest, StretchVideoRequest};
pub use handler_pipeline::{AvatarPipelineRequest, LipSyncRequest, RemoveBackgroundRequest};
pub use routes::routes;
