//! Orchestrator core business logic

// Re-export domain and ports from their dedicated crates
pub use orchestrator_domain as domain;
pub use orchestrator_domain::AppConfig;
pub use orchestrator_ports as ports;
pub use orchestrator_ports::{AvatarPipelineClient, LlmClient, McpClient, TtsClient};

mod asset_ops;
mod avatar_ops;
mod scene_ops;
mod script_ops;
mod service_ops;
pub mod services;

pub use services::OrchestratorApp;
