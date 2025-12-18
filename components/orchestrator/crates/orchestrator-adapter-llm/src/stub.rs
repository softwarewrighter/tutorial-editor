//! Stub LLM client for testing

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_ports_services::LlmClient;

pub struct StubLlmClient;

#[async_trait]
impl LlmClient for StubLlmClient {
    async fn generate_script(&self, _prompt: &str) -> Result<String> {
        Ok("Stub script response".to_string())
    }
}
