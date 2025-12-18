//! Low-level avatar service operations trait definition

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Low-level avatar service operations trait
#[allow(async_fn_in_trait)]
pub trait AvatarServiceOps {
    /// Synthesize speech from text using TTS service
    async fn synthesize_speech(&self, text: &str) -> Result<PathBuf>;
    /// Generate avatar video from image
    async fn generate_avatar_video(&self, image_path: &Path) -> Result<PathBuf>;
    /// Stretch video to target duration
    async fn stretch_video(&self, video_path: &Path, target_duration_ms: u64) -> Result<PathBuf>;
    /// Lip sync video with audio
    async fn lip_sync_video(&self, video_path: &Path, audio_path: &Path) -> Result<PathBuf>;
    /// Remove video background
    async fn remove_video_background(&self, video_path: &Path) -> Result<PathBuf>;
}
