use anyhow::Result;
use async_trait::async_trait;
use orchestrator_core::domain::SceneTarget;
use orchestrator_core::ports::{AvatarPipelineClient, LlmClient, McpClient, TtsClient};
use std::path::{Path, PathBuf};

/// No-op LLM client for testing
pub struct StubLlmClient;

#[async_trait]
impl LlmClient for StubLlmClient {
    async fn generate_script(&self, _prompt: &str) -> Result<String> {
        Ok("Stub script response".to_string())
    }
}

/// No-op TTS client for testing
pub struct StubTtsClient;

#[async_trait]
impl TtsClient for StubTtsClient {
    async fn synthesize(&self, _text: &str) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-audio.wav"))
    }
}

/// No-op Avatar pipeline client for testing
pub struct StubAvatarClient;

#[async_trait]
impl AvatarPipelineClient for StubAvatarClient {
    async fn generate_video(&self, _image_path: &Path) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-avatar.mp4"))
    }

    async fn lip_sync(&self, _video: &Path, _audio: &Path) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-lipsync.mp4"))
    }

    async fn remove_background(&self, _video: &Path) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-nobg.mp4"))
    }
}

/// No-op MCP client for testing
pub struct StubMcpClient;

#[async_trait]
impl McpClient for StubMcpClient {
    async fn capture_scene(&self, _target: &SceneTarget) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-capture.mp4"))
    }
}
