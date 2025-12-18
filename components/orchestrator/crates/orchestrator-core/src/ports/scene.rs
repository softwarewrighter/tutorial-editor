use crate::domain::Scene;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SceneRepository: Send + Sync {
    async fn list_scenes_by_project(&self, project_id: i64) -> Result<Vec<Scene>>;
    async fn get_scene(&self, id: i64) -> Result<Option<Scene>>;
    async fn create_scene(&self, scene: Scene) -> Result<Scene>;
    async fn update_scene(&self, scene: Scene) -> Result<Scene>;
    async fn delete_scene(&self, id: i64) -> Result<()>;
}
