//! Service client trait definitions for external services

mod avatar;
mod mcp;
mod text;

pub use avatar::AvatarPipelineClient;
pub use mcp::McpClient;
pub use text::{LlmClient, TtsClient};
