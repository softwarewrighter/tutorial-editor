//! Service client traits for external services

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::SceneTarget;
use std::path::{Path, PathBuf};

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate_script(&self, prompt: &str) -> Result<String>;
}

#[async_trait]
pub trait TtsClient: Send + Sync {
    async fn synthesize(&self, text: &str) -> Result<PathBuf>;
}

#[async_trait]
pub trait AvatarPipelineClient: Send + Sync {
    async fn generate_video(&self, image_path: &Path) -> Result<PathBuf>;
    async fn stretch_video(&self, video: &Path, target_duration_ms: u64) -> Result<PathBuf>;
    async fn lip_sync(&self, video: &Path, audio: &Path) -> Result<PathBuf>;
    async fn remove_background(&self, video: &Path) -> Result<PathBuf>;
}

#[async_trait]
pub trait McpClient: Send + Sync {
    async fn capture_scene(&self, target: &SceneTarget) -> Result<PathBuf>;
}
