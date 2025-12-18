use crate::{
    domain::Asset,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
    services::OrchestratorApp,
};
use anyhow::{anyhow, Result};
use std::path::Path;

impl<P, S, A> OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    /// Generate TTS audio and create an asset
    pub async fn avatar_generate_audio(&self, scene_id: i64, text: &str) -> Result<Asset> {
        let scene = self.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;
        let audio_path = self.synthesize_speech(text).await?;
        let asset = Asset::new(
            scene.project_id,
            Some(scene_id),
            format!("tts_audio_{}", scene_id),
            "tts_audio".to_string(),
            Some(audio_path.to_string_lossy().to_string()),
            None,
            None,
        );
        self.create_asset(asset).await
    }

    /// Generate avatar video from image asset
    pub async fn avatar_generate_video(&self, scene_id: i64, image_asset_id: i64) -> Result<Asset> {
        let scene = self.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;
        let image = self.get_asset(image_asset_id).await?.ok_or_else(|| anyhow!("Image not found"))?;
        let image_path = image.file_path.ok_or_else(|| anyhow!("Image has no file path"))?;
        let video_path = self.generate_avatar_video(Path::new(&image_path)).await?;
        let asset = Asset::new(
            scene.project_id,
            Some(scene_id),
            format!("avatar_video_{}", scene_id),
            "avatar_video".to_string(),
            Some(video_path.to_string_lossy().to_string()),
            None,
            None,
        );
        self.create_asset(asset).await
    }

    /// Stretch video to target duration
    pub async fn avatar_stretch_video(
        &self,
        scene_id: i64,
        video_asset_id: i64,
        target_duration_ms: u64,
    ) -> Result<Asset> {
        let scene = self.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;
        let video = self.get_asset(video_asset_id).await?.ok_or_else(|| anyhow!("Video not found"))?;
        let video_path = video.file_path.ok_or_else(|| anyhow!("Video has no file path"))?;
        let stretched = self.stretch_video(Path::new(&video_path), target_duration_ms).await?;
        let asset = Asset::new(
            scene.project_id,
            Some(scene_id),
            format!("avatar_video_stretched_{}", scene_id),
            "avatar_video_stretched".to_string(),
            Some(stretched.to_string_lossy().to_string()),
            None,
            Some(format!(r#"{{"target_duration_ms":{}}}"#, target_duration_ms)),
        );
        self.create_asset(asset).await
    }

    /// Lip sync video with audio
    pub async fn avatar_lip_sync(
        &self,
        scene_id: i64,
        video_asset_id: i64,
        audio_asset_id: i64,
    ) -> Result<Asset> {
        let scene = self.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;
        let video = self.get_asset(video_asset_id).await?.ok_or_else(|| anyhow!("Video not found"))?;
        let audio = self.get_asset(audio_asset_id).await?.ok_or_else(|| anyhow!("Audio not found"))?;
        let video_path = video.file_path.ok_or_else(|| anyhow!("Video has no file path"))?;
        let audio_path = audio.file_path.ok_or_else(|| anyhow!("Audio has no file path"))?;
        let synced = self.lip_sync_video(Path::new(&video_path), Path::new(&audio_path)).await?;
        let asset = Asset::new(
            scene.project_id,
            Some(scene_id),
            format!("avatar_lipsync_{}", scene_id),
            "avatar_lipsync".to_string(),
            Some(synced.to_string_lossy().to_string()),
            None,
            None,
        );
        self.create_asset(asset).await
    }

    /// Remove background from video
    pub async fn avatar_remove_background(&self, scene_id: i64, video_asset_id: i64) -> Result<Asset> {
        let scene = self.get_scene(scene_id).await?.ok_or_else(|| anyhow!("Scene not found"))?;
        let video = self.get_asset(video_asset_id).await?.ok_or_else(|| anyhow!("Video not found"))?;
        let video_path = video.file_path.ok_or_else(|| anyhow!("Video has no file path"))?;
        let nobg = self.remove_video_background(Path::new(&video_path)).await?;
        let asset = Asset::new(
            scene.project_id,
            Some(scene_id),
            format!("avatar_final_{}", scene_id),
            "avatar_final".to_string(),
            Some(nobg.to_string_lossy().to_string()),
            None,
            None,
        );
        self.create_asset(asset).await
    }

    /// Run the full avatar pipeline: audio -> video -> stretch -> lip-sync -> remove background
    pub async fn avatar_pipeline(
        &self,
        scene_id: i64,
        image_asset_id: i64,
        script_text: &str,
        target_duration_ms: u64,
    ) -> Result<Vec<Asset>> {
        let mut assets = Vec::new();

        // 1. Generate TTS audio
        let audio_asset = self.avatar_generate_audio(scene_id, script_text).await?;
        let audio_id = audio_asset.id.ok_or_else(|| anyhow!("Audio asset has no ID"))?;
        assets.push(audio_asset);

        // 2. Generate avatar video from image
        let video_asset = self.avatar_generate_video(scene_id, image_asset_id).await?;
        let video_id = video_asset.id.ok_or_else(|| anyhow!("Video asset has no ID"))?;
        assets.push(video_asset);

        // 3. Stretch video to match audio duration
        let stretched = self.avatar_stretch_video(scene_id, video_id, target_duration_ms).await?;
        let stretched_id = stretched.id.ok_or_else(|| anyhow!("Stretched asset has no ID"))?;
        assets.push(stretched);

        // 4. Lip sync stretched video with audio
        let lipsync = self.avatar_lip_sync(scene_id, stretched_id, audio_id).await?;
        let lipsync_id = lipsync.id.ok_or_else(|| anyhow!("Lipsync asset has no ID"))?;
        assets.push(lipsync);

        // 5. Remove background
        let final_asset = self.avatar_remove_background(scene_id, lipsync_id).await?;
        assets.push(final_asset);

        Ok(assets)
    }
}
