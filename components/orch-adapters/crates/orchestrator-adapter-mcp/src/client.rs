//! HTTP MCP client implementation

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::SceneTarget;
use orchestrator_ports_services::McpClient;
use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;

pub struct HttpMcpClient {
    client: Client,
    base_url: String,
}

impl HttpMcpClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[derive(Deserialize)]
struct CaptureResponse {
    capture_path: String,
}

#[async_trait]
impl McpClient for HttpMcpClient {
    async fn capture_scene(&self, target: &SceneTarget) -> Result<PathBuf> {
        let url = format!("{}/capture", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(target)
            .send()
            .await?
            .error_for_status()?
            .json::<CaptureResponse>()
            .await?;

        Ok(PathBuf::from(response.capture_path))
    }
}
