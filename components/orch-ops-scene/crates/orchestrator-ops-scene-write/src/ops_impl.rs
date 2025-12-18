//! Scene write operations implementation

use crate::{SceneOps, SceneWriteOps};
use anyhow::Result;
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_domain::Scene;
use orchestrator_ops_scene_read::SceneReadOps;

impl<P, S, A> SceneWriteOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn create_scene(&self, project_id: i64, title: String, order: i32) -> Result<Scene> {
        self.scene_repo.create_scene(Scene::new(project_id, title, order)).await
    }

    async fn update_scene(&self, scene: Scene) -> Result<Scene> {
        self.scene_repo.update_scene(scene).await
    }

    async fn delete_scene(&self, id: i64) -> Result<()> {
        self.scene_repo.delete_scene(id).await
    }
}

impl<P, S, A> SceneOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
}
