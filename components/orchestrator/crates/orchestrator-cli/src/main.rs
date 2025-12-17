use anyhow::Result;
use clap::Parser;
use orchestrator_core::{AppConfig, OrchestratorApp};
use orchestrator_db::SqliteProjectRepository;
use orchestrator_http::HttpServer;
use std::fs;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(name = "orchestrator-cli")]
#[command(about = "CLI + HTTP server for AI-assisted video orchestration", long_about = None)]
struct CliArgs {
    /// Path to configuration file
    #[arg(short = 'c', long = "config-file", default_value = "./config.toml")]
    config_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting orchestrator-cli with config {:?}", args.config_file);

    let config = load_config(&args.config_file)?;
    let repo = SqliteProjectRepository::new(&config.storage.sqlite_path)?;
    let app = Arc::new(OrchestratorApp::new(config.clone(), Arc::new(repo)));

    let http = HttpServer::new(app);
    http.run(&config.server.bind_address, config.server.port).await?;

    Ok(())
}

fn load_config(path: &str) -> Result<AppConfig> {
    let raw = fs::read_to_string(path)?;
    let cfg: AppConfig = toml::from_str(&raw)?;
    Ok(cfg)
}
