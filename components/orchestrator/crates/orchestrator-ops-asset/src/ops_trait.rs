//! Asset operations trait definition - re-exports

pub use crate::ops_trait_read::AssetReadOps;
pub use crate::ops_trait_write::AssetWriteOps;

/// Combined asset operations (for convenience)
pub trait AssetOps: AssetReadOps + AssetWriteOps {}
