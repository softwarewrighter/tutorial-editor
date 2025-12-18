//! Avatar pipeline operations trait definition

use anyhow::Result;
use orchestrator_avatar_media::AvatarMediaOps;
use orchestrator_domain::Asset;

/// Avatar pipeline operations trait
#[allow(async_fn_in_trait)]
pub trait AvatarPipelineOps: AvatarMediaOps {
    /// Lip sync and create asset
    async fn avatar_lip_sync(&self, scene_id: i64, video_asset_id: i64, audio_asset_id: i64) -> Result<Asset>;
    /// Remove background and create asset
    async fn avatar_remove_background(&self, scene_id: i64, video_asset_id: i64) -> Result<Asset>;
    /// Run full avatar pipeline
    async fn avatar_pipeline(&self, scene_id: i64, image_asset_id: i64, script_text: &str, target_duration_ms: u64) -> Result<Vec<Asset>>;
}
