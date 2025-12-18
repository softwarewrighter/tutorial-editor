use crate::domain::{GeneratedScript, Scene};
use crate::ports::{AssetRepository, ProjectRepository, SceneRepository};
use crate::prompts::{build_project_script_prompt, build_scene_script_prompt};
use crate::OrchestratorApp;
use anyhow::{anyhow, Result};

impl<P, S, A> OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    /// Generate a full project script and create scenes from it
    pub async fn generate_project_script(
        &self,
        project_id: i64,
        user_prompt: &str,
    ) -> Result<Vec<Scene>> {
        let project = self
            .get_project(project_id)
            .await?
            .ok_or_else(|| anyhow!("Project not found: {}", project_id))?;

        let prompt = build_project_script_prompt(&project, user_prompt);
        let response = self.generate_script(&prompt).await?;
        let generated: GeneratedScript = serde_json::from_str(&response)
            .map_err(|e| anyhow!("Failed to parse LLM response: {}", e))?;

        let mut scenes = Vec::new();
        for gs in generated.scenes {
            let mut scene = Scene::new(project_id, gs.title, gs.order);
            scene.script_text = Some(gs.script_text);
            scene.description = gs.description;
            let created = self.scene_repo.create_scene(scene).await?;
            scenes.push(created);
        }

        Ok(scenes)
    }

    /// Generate or refine a single scene's script
    pub async fn generate_scene_script(
        &self,
        scene_id: i64,
        user_prompt: &str,
    ) -> Result<Scene> {
        let mut scene = self
            .get_scene(scene_id)
            .await?
            .ok_or_else(|| anyhow!("Scene not found: {}", scene_id))?;

        let project = self
            .get_project(scene.project_id)
            .await?
            .ok_or_else(|| anyhow!("Project not found: {}", scene.project_id))?;

        let prompt = build_scene_script_prompt(&scene, &project, user_prompt);
        let response = self.generate_script(&prompt).await?;

        scene.script_text = Some(response.trim().to_string());
        let updated = self.scene_repo.update_scene(scene).await?;

        Ok(updated)
    }
}
