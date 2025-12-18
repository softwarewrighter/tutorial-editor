use anyhow::Result;
use async_trait::async_trait;
use orchestrator_core::ports::TtsClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub struct HttpTtsClient {
    client: Client,
    base_url: String,
}

impl HttpTtsClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[derive(Serialize)]
struct SynthesizeRequest<'a> {
    text: &'a str,
}

#[derive(Deserialize)]
struct SynthesizeResponse {
    audio_path: String,
}

#[async_trait]
impl TtsClient for HttpTtsClient {
    async fn synthesize(&self, text: &str) -> Result<PathBuf> {
        let url = format!("{}/synthesize", self.base_url);
        let request = SynthesizeRequest { text };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<SynthesizeResponse>()
            .await?;

        Ok(PathBuf::from(response.audio_path))
    }
}
