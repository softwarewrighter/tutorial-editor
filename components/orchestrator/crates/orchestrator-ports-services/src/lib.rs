//! Service client trait definitions for external services

mod clients;

pub use clients::{AvatarPipelineClient, LlmClient, McpClient, TtsClient};
