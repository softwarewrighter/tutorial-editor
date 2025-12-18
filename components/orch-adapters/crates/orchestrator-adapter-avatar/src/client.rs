//! HTTP Avatar pipeline client implementation

use crate::http::post_video_request;
use crate::HttpAvatarClient;
use anyhow::Result;
use async_trait::async_trait;
use orchestrator_ports_services::AvatarPipelineClient;
use serde::Serialize;
use std::path::{Path, PathBuf};

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
