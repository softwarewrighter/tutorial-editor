//! Stub MCP client for testing

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::SceneTarget;
use orchestrator_ports_services::McpClient;
use std::path::PathBuf;

pub struct StubMcpClient;

#[async_trait]
impl McpClient for StubMcpClient {
    async fn capture_scene(&self, _target: &SceneTarget) -> Result<PathBuf> {
        Ok(PathBuf::from("/tmp/stub-capture.mp4"))
    }
}
