use crate::{
    domain::Scene,
    ports::{ProjectRepository, SceneRepository},
    services::OrchestratorApp,
};
use anyhow::Result;

impl<P, S> OrchestratorApp<P, S>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    pub async fn list_scenes(&self, project_id: i64) -> Result<Vec<Scene>> {
        self.scene_repo.list_scenes_by_project(project_id).await
    }

    pub async fn get_scene(&self, id: i64) -> Result<Option<Scene>> {
        self.scene_repo.get_scene(id).await
    }

    pub async fn create_scene(&self, project_id: i64, title: String, sort: i32) -> Result<Scene> {
        let scene = Scene::new(project_id, title, sort);
        self.scene_repo.create_scene(scene).await
    }

    pub async fn update_scene(&self, scene: Scene) -> Result<Scene> {
        self.scene_repo.update_scene(scene).await
    }

    pub async fn delete_scene(&self, id: i64) -> Result<()> {
        self.scene_repo.delete_scene(id).await
    }
}
