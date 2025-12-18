//! HTTP helper for avatar client

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct VideoResponse {
    pub output_path: String,
}

pub async fn post_video_request<T: Serialize>(
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
