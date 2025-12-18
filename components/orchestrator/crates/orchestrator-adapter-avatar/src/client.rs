//! HTTP Avatar pipeline client implementation

use anyhow::Result;
use async_trait::async_trait;
use orchestrator_domain::AvatarServices;
use orchestrator_ports_services::AvatarPipelineClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub struct HttpAvatarClient {
    client: Client,
    config: AvatarServices,
}

impl HttpAvatarClient {
    pub fn new(config: AvatarServices) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
}

#[derive(Serialize)]
struct ImageToVideoRequest<'a> {
    image_path: &'a str,
}

#[derive(Serialize)]
struct LipSyncRequest<'a> {
    video_path: &'a str,
    audio_path: &'a str,
}

#[derive(Serialize)]
struct BackgroundRemovalRequest<'a> {
    video_path: &'a str,
}

#[derive(Serialize)]
struct VideoStretchRequest<'a> {
    video_path: &'a str,
    target_duration_ms: u64,
}

#[derive(Deserialize)]
struct VideoResponse {
    output_path: String,
}

#[async_trait]
impl AvatarPipelineClient for HttpAvatarClient {
    async fn generate_video(&self, image_path: &Path) -> Result<PathBuf> {
        let url = format!("{}/generate", self.config.image_to_video);
        let request = ImageToVideoRequest {
            image_path: image_path.to_str().unwrap_or_default(),
        };
        post_video_request(&self.client, &url, &request).await
    }

    async fn stretch_video(&self, video: &Path, target_duration_ms: u64) -> Result<PathBuf> {
        let url = format!("{}/stretch", self.config.video_stretch);
        let request = VideoStretchRequest {
            video_path: video.to_str().unwrap_or_default(),
            target_duration_ms,
        };
        post_video_request(&self.client, &url, &request).await
    }

    async fn lip_sync(&self, video: &Path, audio: &Path) -> Result<PathBuf> {
        let url = format!("{}/sync", self.config.lip_sync);
        let request = LipSyncRequest {
            video_path: video.to_str().unwrap_or_default(),
            audio_path: audio.to_str().unwrap_or_default(),
        };
        post_video_request(&self.client, &url, &request).await
    }

    async fn remove_background(&self, video: &Path) -> Result<PathBuf> {
        let url = format!("{}/remove", self.config.background_removal);
        let request = BackgroundRemovalRequest {
            video_path: video.to_str().unwrap_or_default(),
        };
        post_video_request(&self.client, &url, &request).await
    }
}

async fn post_video_request<T: Serialize>(
    client: &Client,
    url: &str,
    request: &T,
) -> Result<PathBuf> {
    let response = client
        .post(url)
        .json(request)
        .send()
        .await?
        .error_for_status()?
        .json::<VideoResponse>()
        .await?;
    Ok(PathBuf::from(response.output_path))
}
