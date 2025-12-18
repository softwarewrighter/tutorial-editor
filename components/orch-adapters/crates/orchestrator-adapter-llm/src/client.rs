//! HTTP LLM client implementation

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_ports_services::LlmClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct HttpLlmClient {
    client: Client,
    base_url: String,
}

impl HttpLlmClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[derive(Serialize)]
struct GenerateRequest<'a> {
    prompt: &'a str,
}

#[derive(Deserialize)]
struct GenerateResponse {
    text: String,
}

#[async_trait]
impl LlmClient for HttpLlmClient {
    async fn generate_script(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/generate", self.base_url);
        let request = GenerateRequest { prompt };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<GenerateResponse>()
            .await?;

        Ok(response.text)
    }
}
