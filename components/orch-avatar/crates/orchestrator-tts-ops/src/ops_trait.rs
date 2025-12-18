//! TTS operations trait

use anyhow::Result;
use std::path::PathBuf;

#[allow(async_fn_in_trait)]
pub trait TtsOps {
    async fn synthesize_speech(&self, text: &str) -> Result<PathBuf>;
}
