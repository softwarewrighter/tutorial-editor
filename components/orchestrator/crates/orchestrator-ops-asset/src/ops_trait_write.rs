//! Asset write operations trait definition

use anyhow::Result;
use orchestrator_domain::Asset;

/// Asset write operations trait
#[allow(async_fn_in_trait)]
pub trait AssetWriteOps {
    /// Create a new asset
    async fn create_asset(&self, asset: Asset) -> Result<Asset>;
    /// Update an existing asset
    async fn update_asset(&self, asset: Asset) -> Result<Asset>;
    /// Delete an asset
    async fn delete_asset(&self, id: i64) -> Result<()>;
}
