//! Scene read operations implementation

use crate::SceneReadOps;
use anyhow::Result;
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_domain::Scene;

impl<P, S, A> SceneReadOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn list_scenes_by_project(&self, project_id: i64) -> Result<Vec<Scene>> {
        self.scene_repo.list_scenes_by_project(project_id).await
    }

    async fn get_scene(&self, id: i64) -> Result<Option<Scene>> {
        self.scene_repo.get_scene(id).await
    }
}
