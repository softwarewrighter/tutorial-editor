use crate::domain::SceneTarget;
use crate::ports::{AssetRepository, ProjectRepository, SceneRepository};
use crate::OrchestratorApp;
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

impl<P, S, A> OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    pub async fn generate_script(&self, prompt: &str) -> Result<String> {
        let client = self
            .llm_client
            .as_ref()
            .ok_or_else(|| anyhow!("LLM client not configured"))?;
        client.generate_script(prompt).await
    }

    pub async fn synthesize_speech(&self, text: &str) -> Result<PathBuf> {
        let client = self
            .tts_client
            .as_ref()
            .ok_or_else(|| anyhow!("TTS client not configured"))?;
        client.synthesize(text).await
    }

    pub async fn generate_avatar_video(&self, image_path: &Path) -> Result<PathBuf> {
        let client = self
            .avatar_client
            .as_ref()
            .ok_or_else(|| anyhow!("Avatar client not configured"))?;
        client.generate_video(image_path).await
    }

    pub async fn lip_sync_video(&self, video: &Path, audio: &Path) -> Result<PathBuf> {
        let client = self
            .avatar_client
            .as_ref()
            .ok_or_else(|| anyhow!("Avatar client not configured"))?;
        client.lip_sync(video, audio).await
    }

    pub async fn remove_video_background(&self, video: &Path) -> Result<PathBuf> {
        let client = self
            .avatar_client
            .as_ref()
            .ok_or_else(|| anyhow!("Avatar client not configured"))?;
        client.remove_background(video).await
    }

    pub async fn stretch_video(&self, video: &Path, target_duration_ms: u64) -> Result<PathBuf> {
        let client = self
            .avatar_client
            .as_ref()
            .ok_or_else(|| anyhow!("Avatar client not configured"))?;
        client.stretch_video(video, target_duration_ms).await
    }

    pub async fn capture_scene(&self, target: &SceneTarget) -> Result<PathBuf> {
        let client = self
            .mcp_client
            .as_ref()
            .ok_or_else(|| anyhow!("MCP client not configured"))?;
        client.capture_scene(target).await
    }
}
