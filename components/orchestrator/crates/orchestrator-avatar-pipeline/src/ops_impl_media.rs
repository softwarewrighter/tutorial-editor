//! Avatar media operations implementation

use anyhow::{anyhow, Result};
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_avatar_service::{AvatarServiceOps, TtsOps};
use orchestrator_domain::Asset;
use orchestrator_ops_asset::{AssetReadOps, AssetWriteOps};
use orchestrator_ops_scene::SceneReadOps;
use std::path::Path;

use crate::AvatarMediaOps;

/// Convert PathBuf to Option<String> for asset file_path
pub fn path_to_string(path: std::path::PathBuf) -> Option<String> {
    Some(path.to_string_lossy().into_owned())
}

impl<P, S, A> AvatarMediaOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn avatar_generate_audio(&self, scene_id: i64, text: &str) -> Result<Asset> {
        let audio_path = self.synthesize_speech(text).await?;
        let scene = self.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;
        let asset = Asset::new(scene.project_id, Some(scene_id), "tts_audio".into(), "tts_audio".into(), path_to_string(audio_path), None, None);
        self.create_asset(asset).await
    }

    async fn avatar_generate_video(&self, scene_id: i64, image_asset_id: i64) -> Result<Asset> {
        let image_asset = self.get_asset(image_asset_id).await?.ok_or_else(|| anyhow!("Image asset not found"))?;
        let image_path = image_asset.file_path.as_ref().ok_or_else(|| anyhow!("Image asset has no file path"))?;
        let video_path = self.generate_avatar_video(Path::new(image_path)).await?;
        let scene = self.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;
        let asset = Asset::new(scene.project_id, Some(scene_id), "avatar_video".into(), "avatar_video".into(), path_to_string(video_path), None, None);
        self.create_asset(asset).await
    }

    async fn avatar_stretch_video(&self, scene_id: i64, video_asset_id: i64, target_duration_ms: u64) -> Result<Asset> {
        let video_asset = self.get_asset(video_asset_id).await?.ok_or_else(|| anyhow!("Video asset not found"))?;
        let video_path = video_asset.file_path.as_ref().ok_or_else(|| anyhow!("Video asset has no file path"))?;
        let stretched_path = self.stretch_video(Path::new(video_path), target_duration_ms).await?;
        let scene = self.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;
        let asset = Asset::new(scene.project_id, Some(scene_id), "avatar_video_stretched".into(), "avatar_video_stretched".into(), path_to_string(stretched_path), None, None);
        self.create_asset(asset).await
    }
}
