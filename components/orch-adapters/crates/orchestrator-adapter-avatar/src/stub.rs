//! Stub Avatar pipeline client for testing

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_ports_services::AvatarPipelineClient;
use std::path::{Path, PathBuf};

pub struct StubAvatarClient;

#[async_trait]
impl AvatarPipelineClient for StubAvatarClient {
    async fn generate_video(&self, _image_path: &Path) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-avatar.mp4"))
    }

    async fn stretch_video(&self, _video: &Path, _target_duration_ms: u64) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-stretched.mp4"))
    }

    async fn lip_sync(&self, _video: &Path, _audio: &Path) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-lipsync.mp4"))
    }

    async fn remove_background(&self, _video: &Path) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-nobg.mp4"))
    }
}
