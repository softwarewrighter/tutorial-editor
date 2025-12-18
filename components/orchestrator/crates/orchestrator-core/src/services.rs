use crate::{
    config::AppConfig,
    ports::{
        AssetRepository, AvatarPipelineClient, LlmClient, McpClient, ProjectRepository,
        SceneRepository, TtsClient,
    },
};
use std::sync::Arc;

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
    // Optional service clients (trait objects for flexibility)
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

    pub fn with_llm(mut self, client: Arc<dyn LlmClient>) -> Self {
        self.llm_client = Some(client);
        self
    }

    pub fn with_tts(mut self, client: Arc<dyn TtsClient>) -> Self {
        self.tts_client = Some(client);
        self
    }

    pub fn with_avatar(mut self, client: Arc<dyn AvatarPipelineClient>) -> Self {
        self.avatar_client = Some(client);
        self
    }

    pub fn with_mcp(mut self, client: Arc<dyn McpClient>) -> Self {
        self.mcp_client = Some(client);
        self
    }
}
