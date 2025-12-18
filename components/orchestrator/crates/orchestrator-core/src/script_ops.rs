//! Script generation and project CRUD operations

use crate::domain::{GeneratedScript, Project, Scene};
use crate::ports::{AssetRepository, ProjectRepository, SceneRepository};
use crate::OrchestratorApp;
use anyhow::{anyhow, Result};

// === Prompt Building ===

fn build_project_script_prompt(project: &Project, user_prompt: &str) -> String {
    let description = project.description.as_deref().unwrap_or("No description");
    format!(
        r#"You are a video tutorial script writer. Generate a script for:

Project: {title}
Description: {description}

User Request: {user_prompt}

Generate a JSON response with this exact structure:
{{
  "scenes": [
    {{
      "order": 1,
      "title": "Scene title",
      "script_text": "Narrator dialog for this scene...",
      "description": "Visual description of what happens...",
      "overlay_text": "On-screen text if any"
    }}
  ]
}}

Guidelines:
- Create 3-7 scenes for a 5-minute video
- Each scene should have 30-90 seconds of dialog
- Include clear visual descriptions
- Suggest relevant overlay text for key points
- Return ONLY valid JSON, no markdown code blocks"#,
        title = project.title,
        description = description,
        user_prompt = user_prompt
    )
}

fn build_scene_script_prompt(scene: &Scene, project: &Project, user_prompt: &str) -> String {
    let current_script = scene.script_text.as_deref().unwrap_or("No existing script");
    format!(
        r#"Refine or generate a scene script:

Project: {project_title}
Scene: {scene_title}
Current Script: {current_script}

User Request: {user_prompt}

Return ONLY the refined script text. No JSON wrapper, no explanation.
Just the narrator dialog that should be spoken in this scene."#,
        project_title = project.title,
        scene_title = scene.title,
        current_script = current_script,
        user_prompt = user_prompt
    )
}

// === OrchestratorApp Methods ===

impl<P, S, A> OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    // --- Project Operations ---

    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        self.project_repo.list_projects().await
    }

    pub async fn get_project(&self, id: i64) -> Result<Option<Project>> {
        self.project_repo.get_project(id).await
    }

    pub async fn create_project(&self, slug: String, title: String) -> Result<Project> {
        let project = Project::new(slug, title);
        self.project_repo.create_project(project).await
    }

    // --- Script Generation ---

    pub async fn generate_project_script(
        &self,
        project_id: i64,
        user_prompt: &str,
    ) -> Result<Vec<Scene>> {
        let project = self
            .get_project(project_id)
            .await?
            .ok_or_else(|| anyhow!("Project not found: {project_id}"))?;

        let prompt = build_project_script_prompt(&project, user_prompt);
        let response = self.generate_script(&prompt).await?;
        let generated: GeneratedScript = serde_json::from_str(&response)
            .map_err(|e| anyhow!("Failed to parse LLM response: {e}"))?;

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

    pub async fn generate_scene_script(&self, scene_id: i64, user_prompt: &str) -> Result<Scene> {
        let mut scene = self
            .get_scene(scene_id)
            .await?
            .ok_or_else(|| anyhow!("Scene not found: {scene_id}"))?;

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
