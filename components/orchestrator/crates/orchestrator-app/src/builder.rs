//! Builder pattern methods for OrchestratorApp

use crate::{
    AssetRepository, AvatarPipelineClient, LlmClient, McpClient, OrchestratorApp,
    ProjectRepository, SceneRepository, TtsClient,
};
use std::sync::Arc;

impl<P, S, A> OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    /// Add LLM client
    pub fn with_llm(mut self, client: Arc<dyn LlmClient>) -> Self {
        self.llm_client = Some(client);
        self
    }

    /// Add TTS client
    pub fn with_tts(mut self, client: Arc<dyn TtsClient>) -> Self {
        self.tts_client = Some(client);
        self
    }

    /// Add avatar pipeline client
    pub fn with_avatar(mut self, client: Arc<dyn AvatarPipelineClient>) -> Self {
        self.avatar_client = Some(client);
        self
    }

    /// Add MCP client
    pub fn with_mcp(mut self, client: Arc<dyn McpClient>) -> Self {
        self.mcp_client = Some(client);
        self
    }
}
