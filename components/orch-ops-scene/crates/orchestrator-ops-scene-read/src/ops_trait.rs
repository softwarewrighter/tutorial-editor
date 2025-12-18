//! Scene read operations trait

use anyhow::Result;
use orchestrator_domain::Scene;

#[allow(async_fn_in_trait)]
pub trait SceneReadOps {
    async fn list_scenes_by_project(&self, project_id: i64) -> Result<Vec<Scene>>;
    async fn get_scene(&self, id: i64) -> Result<Option<Scene>>;
}
