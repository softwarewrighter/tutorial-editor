//! Avatar routes

use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_http_core::with_app;
use std::sync::Arc;
use warp::Filter;

use crate::handler::{
    handle_generate_audio, handle_generate_video, handle_stretch_video, GenerateAudioRequest,
    GenerateVideoRequest, StretchVideoRequest,
};
use crate::handler_pipeline::{
    handle_avatar_pipeline, handle_lip_sync, handle_remove_background, AvatarPipelineRequest,
    LipSyncRequest, RemoveBackgroundRequest,
};

/// All avatar routes
pub fn routes<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    audio_video_routes(app.clone()).or(pipeline_routes(app))
}

fn audio_video_routes<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let generate_audio = warp::path!("api" / "scenes" / i64 / "avatar" / "generate-audio")
        .and(warp::post())
        .and(with_app(app.clone()))
        .and(warp::body::json::<GenerateAudioRequest>())
        .and_then(handle_generate_audio);
    let generate_video = warp::path!("api" / "scenes" / i64 / "avatar" / "generate-video")
        .and(warp::post())
        .and(with_app(app.clone()))
        .and(warp::body::json::<GenerateVideoRequest>())
        .and_then(handle_generate_video);
    let stretch_video = warp::path!("api" / "scenes" / i64 / "avatar" / "stretch-video")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<StretchVideoRequest>())
        .and_then(handle_stretch_video);
    generate_audio.or(generate_video).or(stretch_video)
}

fn pipeline_routes<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let lip_sync = warp::path!("api" / "scenes" / i64 / "avatar" / "lip-sync")
        .and(warp::post())
        .and(with_app(app.clone()))
        .and(warp::body::json::<LipSyncRequest>())
        .and_then(handle_lip_sync);
    let remove_bg = warp::path!("api" / "scenes" / i64 / "avatar" / "remove-background")
        .and(warp::post())
        .and(with_app(app.clone()))
        .and(warp::body::json::<RemoveBackgroundRequest>())
        .and_then(handle_remove_background);
    let pipeline = warp::path!("api" / "scenes" / i64 / "avatar" / "pipeline")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<AvatarPipelineRequest>())
        .and_then(handle_avatar_pipeline);
    lip_sync.or(remove_bg).or(pipeline)
}
