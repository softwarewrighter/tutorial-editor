//! Asset read operations

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::Asset;

/// Read operations for assets
#[async_trait]
pub trait AssetReadOps: Send + Sync {
    /// List all assets for a project
    async fn list_assets_by_project(&self, project_id: i64) -> Result<Vec<Asset>>;

    /// List all assets for a scene
    async fn list_assets_by_scene(&self, scene_id: i64) -> Result<Vec<Asset>>;

    /// Get an asset by ID
    async fn get_asset(&self, id: i64) -> Result<Option<Asset>>;
}
