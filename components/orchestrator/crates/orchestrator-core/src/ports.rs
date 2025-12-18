use crate::domain::{Asset, Project, Scene};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn list_projects(&self) -> Result<Vec<Project>>;
    async fn create_project(&self, project: Project) -> Result<Project>;
}

#[async_trait]
pub trait SceneRepository: Send + Sync {
    async fn list_scenes_by_project(&self, project_id: i64) -> Result<Vec<Scene>>;
    async fn get_scene(&self, id: i64) -> Result<Option<Scene>>;
    async fn create_scene(&self, scene: Scene) -> Result<Scene>;
    async fn update_scene(&self, scene: Scene) -> Result<Scene>;
    async fn delete_scene(&self, id: i64) -> Result<()>;
}

#[async_trait]
pub trait AssetRepository: Send + Sync {
    async fn list_assets_by_project(&self, project_id: i64) -> Result<Vec<Asset>>;
    async fn list_assets_by_scene(&self, scene_id: i64) -> Result<Vec<Asset>>;
    async fn get_asset(&self, id: i64) -> Result<Option<Asset>>;
    async fn create_asset(&self, asset: Asset) -> Result<Asset>;
    async fn update_asset(&self, asset: Asset) -> Result<Asset>;
    async fn delete_asset(&self, id: i64) -> Result<()>;
}
