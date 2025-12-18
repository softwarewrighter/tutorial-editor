//! Scene write operations trait

use crate::SceneReadOps;
use anyhow::Result;
use orchestrator_domain::Scene;

/// Scene write operations trait
#[allow(async_fn_in_trait)]
pub trait SceneWriteOps {
    /// Create a new scene
    async fn create_scene(&self, project_id: i64, title: String, sort_order: i32) -> Result<Scene>;
    /// Update an existing scene
    async fn update_scene(&self, scene: Scene) -> Result<Scene>;
    /// Delete a scene
    async fn delete_scene(&self, id: i64) -> Result<()>;
}

/// Combined scene operations (for convenience)
pub trait SceneOps: SceneReadOps + SceneWriteOps {}
