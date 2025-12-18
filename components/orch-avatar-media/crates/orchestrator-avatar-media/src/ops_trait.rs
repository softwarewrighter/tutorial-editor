//! Avatar media operations trait definition

use anyhow::Result;
use orchestrator_domain::Asset;

/// Avatar media generation operations trait
#[allow(async_fn_in_trait)]
pub trait AvatarMediaOps {
    /// Generate TTS audio and create asset
    async fn avatar_generate_audio(&self, scene_id: i64, text: &str) -> Result<Asset>;
    /// Generate avatar video and create asset
    async fn avatar_generate_video(&self, scene_id: i64, image_asset_id: i64) -> Result<Asset>;
    /// Stretch video and create asset
    async fn avatar_stretch_video(&self, scene_id: i64, video_asset_id: i64, target_duration_ms: u64) -> Result<Asset>;
}
