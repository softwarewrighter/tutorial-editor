//! Script generation trait for OrchestratorApp

use anyhow::{anyhow, Result};
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_domain::{GeneratedScene, Scene};

use crate::prompts::{build_project_script_prompt, build_scene_script_prompt};

/// Script generation operations
#[allow(async_fn_in_trait)]
pub trait ScriptOps {
    /// Generate a full project script and create scenes
    async fn generate_project_script(&self, project_id: i64, user_prompt: &str) -> Result<Vec<Scene>>;
    /// Generate or refine a scene script
    async fn generate_scene_script(&self, scene_id: i64, user_prompt: &str) -> Result<Scene>;
}

impl<P, S, A> ScriptOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn generate_project_script(&self, project_id: i64, user_prompt: &str) -> Result<Vec<Scene>> {
        let llm = self.llm_client.as_ref().ok_or_else(|| anyhow!("LLM client not configured"))?;
        let project = self.project_repo.get_project(project_id).await?.ok_or_else(|| anyhow!("Project not found"))?;

        let prompt = build_project_script_prompt(&project, user_prompt);
        let response = llm.generate_script(&prompt).await?;
        let generated: Vec<GeneratedScene> = serde_json::from_str(&response)?;

        let mut scenes = Vec::new();
        for item in generated {
            let mut scene = Scene::new(project_id, item.title, item.order);
            scene.description = item.description;
            scene.script_text = Some(item.script_text);
            let created = self.scene_repo.create_scene(scene).await?;
            scenes.push(created);
        }

        Ok(scenes)
    }

    async fn generate_scene_script(&self, scene_id: i64, user_prompt: &str) -> Result<Scene> {
        let llm = self.llm_client.as_ref().ok_or_else(|| anyhow!("LLM client not configured"))?;
        let mut scene = self.scene_repo.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;

        let prompt = build_scene_script_prompt(&scene.title, scene.script_text.as_deref(), user_prompt);
        let response = llm.generate_script(&prompt).await?;

        scene.script_text = Some(response);
        self.scene_repo.update_scene(scene).await
    }
}
