//! Scene operations implementation

use anyhow::Result;
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_domain::Scene;

use crate::{SceneOps, SceneReadOps, SceneWriteOps};

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

impl<P, S, A> SceneWriteOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn create_scene(&self, project_id: i64, title: String, sort_order: i32) -> Result<Scene> {
        let scene = Scene::new(project_id, title, sort_order);
        self.scene_repo.create_scene(scene).await
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
