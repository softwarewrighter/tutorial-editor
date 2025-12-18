//! Avatar HTTP endpoints

mod handler;
mod routes;

pub use handler::{
    AvatarPipelineRequest, GenerateAudioRequest, GenerateVideoRequest, LipSyncRequest,
    RemoveBackgroundRequest, StretchVideoRequest,
};
pub use routes::routes;
