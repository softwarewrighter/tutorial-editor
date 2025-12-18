use anyhow::Result;
use clap::Parser;
use orchestrator_adapters::{HttpAvatarClient, HttpLlmClient, HttpMcpClient, HttpTtsClient};
use orchestrator_core::{AppConfig, OrchestratorApp};
use orchestrator_db::{SqliteAssetRepository, SqliteProjectRepository, SqliteSceneRepository};
use orchestrator_http::HttpServer;
use std::fs;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

mod version;

const LONG_ABOUT: &str = "\
CLI + HTTP server for AI-assisted video orchestration.

This tool provides a backend server for the Avatar Video Orchestrator,
managing projects, scenes, and assets for automated video production.

AI CODING AGENT INSTRUCTIONS:

  1. STARTING THE SERVER:
     - Ensure config.toml exists with valid settings
     - Run: orchestrator-cli -c ./config.toml
     - Server starts on configured bind_address:port

  2. CONFIGURATION:
     The config.toml file should include:
     - [server]: bind_address, port
     - [storage]: sqlite_path, assets_root
     - [services.*]: URLs for LLM, TTS, avatar, MCP services

  3. API ENDPOINTS:
     - GET    /api/health                   - Health check
     - GET    /api/projects                 - List all projects
     - POST   /api/projects                 - Create a project
     - GET    /api/projects/{id}/scenes     - List scenes in project
     - POST   /api/projects/{id}/scenes     - Create a scene
     - PUT    /api/scenes/{id}              - Update a scene
     - DELETE /api/scenes/{id}              - Delete a scene
     - GET    /api/projects/{id}/assets     - List project assets
     - GET    /api/scenes/{id}/assets       - List scene assets
     - POST   /api/assets                   - Create an asset
     - PUT    /api/assets/{id}              - Update an asset
     - DELETE /api/assets/{id}              - Delete an asset

  4. DEVELOPMENT WORKFLOW:
     - Build: cd components/orchestrator && cargo build
     - Test: cargo test
     - Run: ./scripts/run_orchestrator.sh

  5. ERROR HANDLING:
     - Check logs for detailed error messages
     - Ensure SQLite path directory exists
     - Verify service URLs are reachable";

#[derive(Debug, Parser)]
#[command(name = "orchestrator-cli")]
#[command(about = "CLI + HTTP server for AI-assisted video orchestration")]
#[command(long_about = LONG_ABOUT)]
#[command(version = version::version_string())]
struct CliArgs {
    /// Path to configuration file
    #[arg(short = 'c', long = "config-file", default_value = "./config.toml")]
    config_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();
    init_tracing();
    run_server(&args.config_file).await
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

async fn run_server(config_path: &str) -> Result<()> {
    tracing::info!("Starting orchestrator-cli with config {config_path:?}");

    let config = load_config(config_path)?;
    let project_repo = SqliteProjectRepository::new(&config.storage.sqlite_path)?;
    let conn = project_repo.connection();
    let scene_repo = SqliteSceneRepository::new(conn.clone());
    let asset_repo = SqliteAssetRepository::new(conn);

    // Create service clients
    let llm_client = Arc::new(HttpLlmClient::new(config.services.llm.cloud_primary.clone()));
    let tts_client = Arc::new(HttpTtsClient::new(config.services.tts.primary.clone()));
    let avatar_client = Arc::new(HttpAvatarClient::new(config.services.avatar.clone()));
    let mcp_client = Arc::new(HttpMcpClient::new(config.services.mcp.playwright_hub.clone()));

    let app = Arc::new(
        OrchestratorApp::new(
            config.clone(),
            Arc::new(project_repo),
            Arc::new(scene_repo),
            Arc::new(asset_repo),
        )
        .with_llm(llm_client)
        .with_tts(tts_client)
        .with_avatar(avatar_client)
        .with_mcp(mcp_client),
    );

    let http = HttpServer::new(app);
    http.run(&config.server.bind_address, config.server.port)
        .await
}

fn load_config(path: &str) -> Result<AppConfig> {
    let raw = fs::read_to_string(path)?;
    let cfg: AppConfig = toml::from_str(&raw)?;
    Ok(cfg)
}
