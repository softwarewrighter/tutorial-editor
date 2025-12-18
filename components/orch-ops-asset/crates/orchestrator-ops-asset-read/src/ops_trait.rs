//! Asset read operations trait

use anyhow::Result;
use orchestrator_domain::Asset;

#[allow(async_fn_in_trait)]
pub trait AssetReadOps {
    async fn list_assets_by_project(&self, project_id: i64) -> Result<Vec<Asset>>;
    async fn list_assets_by_scene(&self, scene_id: i64) -> Result<Vec<Asset>>;
    async fn get_asset(&self, id: i64) -> Result<Option<Asset>>;
}
