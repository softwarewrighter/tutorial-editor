//! Text processing service traits (LLM and TTS)

use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;

/// Client for LLM script generation
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Generate script text from a prompt
    async fn generate_script(&self, prompt: &str) -> Result<String>;
}

/// Client for text-to-speech synthesis
#[async_trait]
pub trait TtsClient: Send + Sync {
    /// Synthesize text to audio file
    async fn synthesize(&self, text: &str) -> Result<PathBuf>;
}
