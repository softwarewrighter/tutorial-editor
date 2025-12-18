//! Scene repository traits

mod read;
mod write;

pub use read::SceneReadOps;
pub use write::SceneWriteOps;

use async_trait::async_trait;

/// Combined scene repository trait
#[async_trait]
pub trait SceneRepository: SceneReadOps + SceneWriteOps {}

// Auto-implement SceneRepository for any type that implements both
impl<T: SceneReadOps + SceneWriteOps> SceneRepository for T {}
