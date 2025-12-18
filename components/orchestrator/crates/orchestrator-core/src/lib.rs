pub mod config;
pub mod domain;
pub mod ports;
pub mod prompts;
mod asset_ops;
mod project_ops;
mod scene_ops;
mod script_ops;
mod service_ops;
pub mod services;

pub use config::AppConfig;
pub use ports::{AvatarPipelineClient, LlmClient, McpClient, TtsClient};
pub use services::OrchestratorApp;
