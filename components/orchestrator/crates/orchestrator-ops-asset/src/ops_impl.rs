//! Asset operations implementation

use anyhow::Result;
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_domain::Asset;

use crate::{AssetOps, AssetReadOps, AssetWriteOps};

impl<P, S, A> AssetReadOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn list_assets_by_project(&self, project_id: i64) -> Result<Vec<Asset>> {
        self.asset_repo.list_assets_by_project(project_id).await
    }

    async fn list_assets_by_scene(&self, scene_id: i64) -> Result<Vec<Asset>> {
        self.asset_repo.list_assets_by_scene(scene_id).await
    }

    async fn get_asset(&self, id: i64) -> Result<Option<Asset>> {
        self.asset_repo.get_asset(id).await
    }
}

impl<P, S, A> AssetWriteOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn create_asset(&self, asset: Asset) -> Result<Asset> {
        self.asset_repo.create_asset(asset).await
    }

    async fn update_asset(&self, asset: Asset) -> Result<Asset> {
        self.asset_repo.update_asset(asset).await
    }

    async fn delete_asset(&self, id: i64) -> Result<()> {
        self.asset_repo.delete_asset(id).await
    }
}

impl<P, S, A> AssetOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
}
