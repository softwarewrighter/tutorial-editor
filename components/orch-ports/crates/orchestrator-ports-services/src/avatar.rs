//! Avatar pipeline client trait

use anyhow::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};

/// Client for avatar video generation pipeline
#[async_trait]
pub trait AvatarPipelineClient: Send + Sync {
    /// Generate video from image
    async fn generate_video(&self, image_path: &Path) -> Result<PathBuf>;

    /// Stretch video to target duration
    async fn stretch_video(&self, video: &Path, target_duration_ms: u64) -> Result<PathBuf>;

    /// Lip sync video with audio
    async fn lip_sync(&self, video: &Path, audio: &Path) -> Result<PathBuf>;

    /// Remove video background
    async fn remove_background(&self, video: &Path) -> Result<PathBuf>;
}
