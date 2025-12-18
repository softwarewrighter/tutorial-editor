//! Scene write operations

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::Scene;

/// Write operations for scenes
#[async_trait]
pub trait SceneWriteOps: Send + Sync {
    /// Create a new scene
    async fn create_scene(&self, scene: Scene) -> Result<Scene>;

    /// Update an existing scene
    async fn update_scene(&self, scene: Scene) -> Result<Scene>;

    /// Delete a scene
    async fn delete_scene(&self, id: i64) -> Result<()>;
}
