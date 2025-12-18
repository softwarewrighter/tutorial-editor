//! Asset repository traits

mod read;
mod write;

pub use read::AssetReadOps;
pub use write::AssetWriteOps;

use async_trait::async_trait;

/// Combined asset repository trait
#[async_trait]
pub trait AssetRepository: AssetReadOps + AssetWriteOps {}

// Auto-implement AssetRepository for any type that implements both
impl<T: AssetReadOps + AssetWriteOps> AssetRepository for T {}
