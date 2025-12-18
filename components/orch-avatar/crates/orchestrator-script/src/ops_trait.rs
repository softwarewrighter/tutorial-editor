//! Script generation trait definition

use anyhow::Result;
use orchestrator_domain::Scene;

/// Script generation operations
#[allow(async_fn_in_trait)]
pub trait ScriptOps {
    /// Generate project script and create scenes
    async fn generate_project_script(&self, id: i64, prompt: &str) -> Result<Vec<Scene>>;
    /// Generate or refine a scene script
    async fn generate_scene_script(&self, id: i64, prompt: &str) -> Result<Scene>;
}
