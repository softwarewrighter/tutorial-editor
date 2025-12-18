//! Pipeline request handlers

use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_avatar_pipeline::AvatarPipelineOps;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;

/// Request to lip sync video with audio
#[derive(Debug, Deserialize)]
pub struct LipSyncRequest {
    pub video_asset_id: i64,
    pub audio_asset_id: i64,
}

/// Request to remove video background
#[derive(Debug, Deserialize)]
pub struct RemoveBackgroundRequest {
    pub video_asset_id: i64,
}

/// Request for full avatar pipeline
#[derive(Debug, Deserialize)]
pub struct AvatarPipelineRequest {
    pub image_asset_id: i64,
    pub script_text: String,
    pub target_duration_ms: u64,
}

/// Handle lip sync request
pub async fn handle_lip_sync<P, S, A>(
    scene_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: LipSyncRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    match app
        .avatar_lip_sync(scene_id, payload.video_asset_id, payload.audio_asset_id)
        .await
    {
        Ok(asset) => Ok(warp::reply::json(&asset)),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"error": e.to_string()}))),
    }
}

/// Handle remove background request
pub async fn handle_remove_background<P, S, A>(
    scene_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: RemoveBackgroundRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    match app.avatar_remove_background(scene_id, payload.video_asset_id).await {
        Ok(asset) => Ok(warp::reply::json(&asset)),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"error": e.to_string()}))),
    }
}

/// Handle full pipeline request
pub async fn handle_avatar_pipeline<P, S, A>(
    scene_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: AvatarPipelineRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    match app
        .avatar_pipeline(
            scene_id,
            payload.image_asset_id,
            &payload.script_text,
            payload.target_duration_ms,
        )
        .await
    {
        Ok(assets) => Ok(warp::reply::json(&serde_json::json!({"assets": assets}))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"error": e.to_string()}))),
    }
}
