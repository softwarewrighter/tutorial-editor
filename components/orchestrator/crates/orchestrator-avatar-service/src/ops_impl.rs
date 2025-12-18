//! Low-level avatar service operations implementation

use anyhow::{anyhow, Result};
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use std::path::{Path, PathBuf};

use crate::AvatarServiceOps;

impl<P, S, A> AvatarServiceOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn generate_avatar_video(&self, image_path: &Path) -> Result<PathBuf> {
        let avatar = self.avatar_client.as_ref().ok_or_else(|| anyhow!("Avatar client not configured"))?;
        avatar.generate_video(image_path).await
    }

    async fn stretch_video(&self, video_path: &Path, target_duration_ms: u64) -> Result<PathBuf> {
        let avatar = self.avatar_client.as_ref().ok_or_else(|| anyhow!("Avatar client not configured"))?;
        avatar.stretch_video(video_path, target_duration_ms).await
    }

    async fn lip_sync_video(&self, video_path: &Path, audio_path: &Path) -> Result<PathBuf> {
        let avatar = self.avatar_client.as_ref().ok_or_else(|| anyhow!("Avatar client not configured"))?;
        avatar.lip_sync(video_path, audio_path).await
    }

    async fn remove_video_background(&self, video_path: &Path) -> Result<PathBuf> {
        let avatar = self.avatar_client.as_ref().ok_or_else(|| anyhow!("Avatar client not configured"))?;
        avatar.remove_background(video_path).await
    }
}
