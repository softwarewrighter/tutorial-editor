//! Asset write operations implementation

use crate::{AssetOps, AssetWriteOps};
use anyhow::Result;
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_domain::Asset;

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
