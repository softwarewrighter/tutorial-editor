//! Asset write operations

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::Asset;

/// Write operations for assets
#[async_trait]
pub trait AssetWriteOps: Send + Sync {
    /// Create a new asset
    async fn create_asset(&self, asset: Asset) -> Result<Asset>;

    /// Update an existing asset
    async fn update_asset(&self, asset: Asset) -> Result<Asset>;

    /// Delete an asset
    async fn delete_asset(&self, id: i64) -> Result<()>;
}
