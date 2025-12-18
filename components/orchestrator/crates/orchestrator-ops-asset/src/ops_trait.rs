//! Asset operations trait definition

use anyhow::Result;
use orchestrator_domain::Asset;

/// Asset read operations trait
#[allow(async_fn_in_trait)]
pub trait AssetReadOps {
    /// List assets for a project
    async fn list_assets_by_project(&self, project_id: i64) -> Result<Vec<Asset>>;
    /// List assets for a scene
    async fn list_assets_by_scene(&self, scene_id: i64) -> Result<Vec<Asset>>;
    /// Get an asset by ID
    async fn get_asset(&self, id: i64) -> Result<Option<Asset>>;
}

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

/// Combined asset operations (for convenience)
pub trait AssetOps: AssetReadOps + AssetWriteOps {}
