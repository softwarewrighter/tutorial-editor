//! Avatar request handlers

use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_avatar_pipeline::AvatarMediaOps;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;

/// Request to generate audio from text
#[derive(Debug, Deserialize)]
pub struct GenerateAudioRequest {
    pub script_text: String,
}

/// Request to generate video from image
#[derive(Debug, Deserialize)]
pub struct GenerateVideoRequest {
    pub image_asset_id: i64,
}

/// Request to stretch video duration
#[derive(Debug, Deserialize)]
pub struct StretchVideoRequest {
    pub video_asset_id: i64,
    pub target_duration_ms: u64,
}

/// Handle generate audio request
pub async fn handle_generate_audio<P, S, A>(
    scene_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: GenerateAudioRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    match app.avatar_generate_audio(scene_id, &payload.script_text).await {
        Ok(asset) => Ok(warp::reply::json(&asset)),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"error": e.to_string()}))),
    }
}

/// Handle generate video request
pub async fn handle_generate_video<P, S, A>(
    scene_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: GenerateVideoRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    match app.avatar_generate_video(scene_id, payload.image_asset_id).await {
        Ok(asset) => Ok(warp::reply::json(&asset)),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"error": e.to_string()}))),
    }
}

/// Handle stretch video request
pub async fn handle_stretch_video<P, S, A>(
    scene_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: StretchVideoRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    match app
        .avatar_stretch_video(scene_id, payload.video_asset_id, payload.target_duration_ms)
        .await
    {
        Ok(asset) => Ok(warp::reply::json(&asset)),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"error": e.to_string()}))),
    }
}
