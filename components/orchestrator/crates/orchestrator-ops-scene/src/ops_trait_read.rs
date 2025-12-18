//! Scene read operations trait

use anyhow::Result;
use orchestrator_domain::Scene;

/// Scene read operations trait
#[allow(async_fn_in_trait)]
pub trait SceneReadOps {
    /// List scenes for a project
    async fn list_scenes_by_project(&self, project_id: i64) -> Result<Vec<Scene>>;
    /// Get a scene by ID
    async fn get_scene(&self, id: i64) -> Result<Option<Scene>>;
}
