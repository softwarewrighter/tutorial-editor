//! Scene write operations trait

use anyhow::Result;
use orchestrator_domain::Scene;
use orchestrator_ops_scene_read::SceneReadOps;

#[allow(async_fn_in_trait)]
pub trait SceneWriteOps {
    async fn create_scene(&self, project_id: i64, title: String, sort_order: i32) -> Result<Scene>;
    async fn update_scene(&self, scene: Scene) -> Result<Scene>;
    async fn delete_scene(&self, id: i64) -> Result<()>;
}

pub trait SceneOps: SceneReadOps + SceneWriteOps {}
