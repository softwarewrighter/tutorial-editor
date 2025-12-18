use crate::{
    domain::Asset,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
    services::OrchestratorApp,
};
use anyhow::Result;

impl<P, S, A> OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    pub async fn list_project_assets(&self, project_id: i64) -> Result<Vec<Asset>> {
        self.asset_repo.list_assets_by_project(project_id).await
    }

    pub async fn list_scene_assets(&self, scene_id: i64) -> Result<Vec<Asset>> {
        self.asset_repo.list_assets_by_scene(scene_id).await
    }

    pub async fn get_asset(&self, id: i64) -> Result<Option<Asset>> {
        self.asset_repo.get_asset(id).await
    }

    pub async fn create_asset(&self, asset: Asset) -> Result<Asset> {
        self.asset_repo.create_asset(asset).await
    }

    pub async fn update_asset(&self, asset: Asset) -> Result<Asset> {
        self.asset_repo.update_asset(asset).await
    }

    pub async fn delete_asset(&self, id: i64) -> Result<()> {
        self.asset_repo.delete_asset(id).await
    }
}
