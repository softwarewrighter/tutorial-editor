//! TTS operations implementation

use crate::TtsOps;
use anyhow::{anyhow, Result};
use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use std::path::PathBuf;

impl<P, S, A> TtsOps for OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    async fn synthesize_speech(&self, text: &str) -> Result<PathBuf> {
        let tts = self.tts_client.as_ref().ok_or_else(|| anyhow!("TTS client not configured"))?;
        tts.synthesize(text).await
    }
}
