use anyhow::Result;
use clap::Parser;
use orchestrator_core::{AppConfig, OrchestratorApp};
use orchestrator_db::SqliteProjectRepository;
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
     - GET  /api/health    - Health check
     - GET  /api/projects  - List all projects
     - POST /api/projects  - Create a new project

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
    let repo = SqliteProjectRepository::new(&config.storage.sqlite_path)?;
    let app = Arc::new(OrchestratorApp::new(config.clone(), Arc::new(repo)));

    let http = HttpServer::new(app);
    http.run(&config.server.bind_address, config.server.port).await
}

fn load_config(path: &str) -> Result<AppConfig> {
    let raw = fs::read_to_string(path)?;
    let cfg: AppConfig = toml::from_str(&raw)?;
    Ok(cfg)
}
