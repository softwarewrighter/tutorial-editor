//! Avatar pipeline operations implementation

use anyhow::{anyhow, Result};
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_avatar_service::AvatarServiceOps;
use orchestrator_domain::Asset;
use orchestrator_ops_asset::{AssetReadOps, AssetWriteOps};
use orchestrator_ops_scene::SceneReadOps;
use std::path::Path;

use crate::AvatarPipelineOps;

/// Convert PathBuf to Option<String> for asset file_path
fn path_to_string(path: std::path::PathBuf) -> Option<String> {
    Some(path.to_string_lossy().into_owned())
}

impl<P, S, A> AvatarPipelineOps for OrchestratorApp<P, S, A>
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

    async fn avatar_lip_sync(&self, scene_id: i64, video_asset_id: i64, audio_asset_id: i64) -> Result<Asset> {
        let video_asset = self.get_asset(video_asset_id).await?.ok_or_else(|| anyhow!("Video asset not found"))?;
        let audio_asset = self.get_asset(audio_asset_id).await?.ok_or_else(|| anyhow!("Audio asset not found"))?;
        let video_path = video_asset.file_path.as_ref().ok_or_else(|| anyhow!("Video asset has no file path"))?;
        let audio_path = audio_asset.file_path.as_ref().ok_or_else(|| anyhow!("Audio asset has no file path"))?;
        let synced_path = self.lip_sync_video(Path::new(video_path), Path::new(audio_path)).await?;
        let scene = self.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;
        let asset = Asset::new(scene.project_id, Some(scene_id), "avatar_lipsync".into(), "avatar_lipsync".into(), path_to_string(synced_path), None, None);
        self.create_asset(asset).await
    }

    async fn avatar_remove_background(&self, scene_id: i64, video_asset_id: i64) -> Result<Asset> {
        let video_asset = self.get_asset(video_asset_id).await?.ok_or_else(|| anyhow!("Video asset not found"))?;
        let video_path = video_asset.file_path.as_ref().ok_or_else(|| anyhow!("Video asset has no file path"))?;
        let nobg_path = self.remove_video_background(Path::new(video_path)).await?;
        let scene = self.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;
        let asset = Asset::new(scene.project_id, Some(scene_id), "avatar_final".into(), "avatar_final".into(), path_to_string(nobg_path), None, None);
        self.create_asset(asset).await
    }

    async fn avatar_pipeline(&self, scene_id: i64, image_asset_id: i64, script_text: &str, target_duration_ms: u64) -> Result<Vec<Asset>> {
        let mut assets = Vec::new();

        // Step 1: Generate TTS audio
        let audio = self.avatar_generate_audio(scene_id, script_text).await?;
        let audio_id = audio.id.ok_or_else(|| anyhow!("Audio asset has no ID"))?;
        assets.push(audio);

        // Step 2: Generate avatar video
        let video = self.avatar_generate_video(scene_id, image_asset_id).await?;
        let video_id = video.id.ok_or_else(|| anyhow!("Video asset has no ID"))?;
        assets.push(video);

        // Step 3: Stretch video
        let stretched = self.avatar_stretch_video(scene_id, video_id, target_duration_ms).await?;
        let stretched_id = stretched.id.ok_or_else(|| anyhow!("Stretched video asset has no ID"))?;
        assets.push(stretched);

        // Step 4: Lip sync
        let synced = self.avatar_lip_sync(scene_id, stretched_id, audio_id).await?;
        let synced_id = synced.id.ok_or_else(|| anyhow!("Synced video asset has no ID"))?;
        assets.push(synced);

        // Step 5: Remove background
        let final_asset = self.avatar_remove_background(scene_id, synced_id).await?;
        assets.push(final_asset);

        Ok(assets)
    }
}
