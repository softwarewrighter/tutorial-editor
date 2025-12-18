//! Scene operations trait definition

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
