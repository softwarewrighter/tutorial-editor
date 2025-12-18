//! Scene read operations

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::Scene;

/// Read operations for scenes
#[async_trait]
pub trait SceneReadOps: Send + Sync {
    /// List all scenes for a project
    async fn list_scenes_by_project(&self, project_id: i64) -> Result<Vec<Scene>>;

    /// Get a scene by ID
    async fn get_scene(&self, id: i64) -> Result<Option<Scene>>;
}
