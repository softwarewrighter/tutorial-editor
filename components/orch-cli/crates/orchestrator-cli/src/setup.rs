//! Server setup helpers

use anyhow::Result;
use orchestrator_adapter_avatar::HttpAvatarClient;
use orchestrator_adapter_llm::HttpLlmClient;
use orchestrator_adapter_mcp::HttpMcpClient;
use orchestrator_adapter_tts::HttpTtsClient;
use orchestrator_app::{AppConfig, OrchestratorApp};
use orchestrator_db_asset::SqliteAssetRepository;
use orchestrator_db_project::SqliteProjectRepository;
use orchestrator_db_scene::SqliteSceneRepository;
use std::fs;
use std::sync::Arc;

pub type Repos = (SqliteProjectRepository, SqliteSceneRepository, SqliteAssetRepository);
pub type App = Arc<OrchestratorApp<SqliteProjectRepository, SqliteSceneRepository, SqliteAssetRepository>>;

pub fn load_config(path: &str) -> Result<AppConfig> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

pub fn create_repositories(config: &AppConfig) -> Result<Repos> {
    let p = SqliteProjectRepository::new(&config.storage.sqlite_path)?;
    let c = p.connection();
    Ok((p, SqliteSceneRepository::new(c.clone()), SqliteAssetRepository::new(c)))
}

pub fn build_app(cfg: AppConfig, p: SqliteProjectRepository, s: SqliteSceneRepository, a: SqliteAssetRepository) -> App {
    Arc::new(
        OrchestratorApp::new(cfg.clone(), Arc::new(p), Arc::new(s), Arc::new(a))
            .with_llm(Arc::new(HttpLlmClient::new(cfg.services.llm.cloud_primary.clone())))
            .with_tts(Arc::new(HttpTtsClient::new(cfg.services.tts.primary.clone())))
            .with_avatar(Arc::new(HttpAvatarClient::new(cfg.services.avatar.clone())))
            .with_mcp(Arc::new(HttpMcpClient::new(cfg.services.mcp.playwright_hub.clone()))),
    )
}
