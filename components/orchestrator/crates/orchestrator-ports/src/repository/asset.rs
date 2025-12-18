use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::Asset;

#[async_trait]
pub trait AssetRepository: Send + Sync {
    async fn list_assets_by_project(&self, project_id: i64) -> Result<Vec<Asset>>;
    async fn list_assets_by_scene(&self, scene_id: i64) -> Result<Vec<Asset>>;
    async fn get_asset(&self, id: i64) -> Result<Option<Asset>>;
    async fn create_asset(&self, asset: Asset) -> Result<Asset>;
    async fn update_asset(&self, asset: Asset) -> Result<Asset>;
    async fn delete_asset(&self, id: i64) -> Result<()>;
}
