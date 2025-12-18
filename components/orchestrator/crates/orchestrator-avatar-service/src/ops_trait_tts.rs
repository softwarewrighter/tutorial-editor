//! TTS operations trait definition

use anyhow::Result;
use std::path::PathBuf;

/// TTS service operations trait
#[allow(async_fn_in_trait)]
pub trait TtsOps {
    /// Synthesize speech from text using TTS service
    async fn synthesize_speech(&self, text: &str) -> Result<PathBuf>;
}
