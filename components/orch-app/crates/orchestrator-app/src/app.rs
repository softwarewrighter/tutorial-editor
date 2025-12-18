//! OrchestratorApp struct definition

use crate::{
    AppConfig, AssetRepository, AvatarPipelineClient, LlmClient, McpClient,
    ProjectRepository, SceneRepository, TtsClient,
};
use std::sync::Arc;

/// Main application container holding all dependencies
pub struct OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    pub config: AppConfig,
    pub project_repo: Arc<P>,
    pub scene_repo: Arc<S>,
    pub asset_repo: Arc<A>,
    pub llm_client: Option<Arc<dyn LlmClient>>,
    pub tts_client: Option<Arc<dyn TtsClient>>,
    pub avatar_client: Option<Arc<dyn AvatarPipelineClient>>,
    pub mcp_client: Option<Arc<dyn McpClient>>,
}

impl<P, S, A> OrchestratorApp<P, S, A>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    /// Create a new OrchestratorApp with required dependencies
    pub fn new(
        config: AppConfig,
        project_repo: Arc<P>,
        scene_repo: Arc<S>,
        asset_repo: Arc<A>,
    ) -> Self {
        Self {
            config,
            project_repo,
            scene_repo,
            asset_repo,
            llm_client: None,
            tts_client: None,
            avatar_client: None,
            mcp_client: None,
        }
    }
}
