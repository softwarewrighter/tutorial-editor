//! MCP client trait

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::SceneTarget;
use std::path::PathBuf;

/// Client for MCP scene capture
#[async_trait]
pub trait McpClient: Send + Sync {
    /// Capture a scene to file
    async fn capture_scene(&self, target: &SceneTarget) -> Result<PathBuf>;
}
