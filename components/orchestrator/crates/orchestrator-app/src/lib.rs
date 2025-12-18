//! Orchestrator application container

mod app;
mod builder;

pub use app::OrchestratorApp;

// Re-export commonly used types
pub use orchestrator_domain as domain;
pub use orchestrator_domain::AppConfig;

/// Port traits re-exported for convenience
pub mod ports {
    pub use orchestrator_ports_asset::{AssetReadOps, AssetRepository, AssetWriteOps};
    pub use orchestrator_ports_project::ProjectRepository;
    pub use orchestrator_ports_scene::{SceneReadOps, SceneRepository, SceneWriteOps};
    pub use orchestrator_ports_services::{AvatarPipelineClient, LlmClient, McpClient, TtsClient};
}

// Also export at top level for direct imports
pub use ports::*;
