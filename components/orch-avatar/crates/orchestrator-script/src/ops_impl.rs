//! Script generation trait implementation

use anyhow::{anyhow, Result};
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_domain::{GeneratedScene, Scene};

use crate::prompts::{build_project_script_prompt, build_scene_script_prompt};
use crate::ScriptOps;

fn to_scene(id: i64, g: GeneratedScene) -> Scene {
    let mut s = Scene::new(id, g.title, g.order);
    s.description = g.description;
    s.script_text = Some(g.script_text);
    s
}

impl<P, S, A> ScriptOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn generate_project_script(&self, project_id: i64, user_prompt: &str) -> Result<Vec<Scene>> {
        let llm = self.llm_client.as_ref().ok_or_else(|| anyhow!("LLM not configured"))?;
        let p = self.project_repo.get_project(project_id).await?.ok_or_else(|| anyhow!("Not found"))?;
        let resp = llm.generate_script(&build_project_script_prompt(&p, user_prompt)).await?;
        let items: Vec<GeneratedScene> = serde_json::from_str(&resp)?;
        let mut out = Vec::new();
        for g in items { out.push(self.scene_repo.create_scene(to_scene(project_id, g)).await?); }
        Ok(out)
    }

    async fn generate_scene_script(&self, scene_id: i64, user_prompt: &str) -> Result<Scene> {
        let llm = self.llm_client.as_ref().ok_or_else(|| anyhow!("LLM not configured"))?;
        let mut s = self.scene_repo.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Not found"))?;
        let prompt = build_scene_script_prompt(&s.title, s.script_text.as_deref(), user_prompt);
        s.script_text = Some(llm.generate_script(&prompt).await?);
        self.scene_repo.update_scene(s).await
    }
}
