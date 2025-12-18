//! Asset operations for OrchestratorApp

mod ops_impl;
mod ops_trait;
mod ops_trait_read;
mod ops_trait_write;

pub use ops_trait::AssetOps;
pub use ops_trait_read::AssetReadOps;
pub use ops_trait_write::AssetWriteOps;
