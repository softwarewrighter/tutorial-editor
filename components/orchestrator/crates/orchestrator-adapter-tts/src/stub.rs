//! Stub TTS client for testing

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_ports_services::TtsClient;
use std::path::PathBuf;

pub struct StubTtsClient;

#[async_trait]
impl TtsClient for StubTtsClient {
    async fn synthesize(&self, _text: &str) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-audio.wav"))
    }
}
