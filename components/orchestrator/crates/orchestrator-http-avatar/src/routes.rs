//! Avatar routes

use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_http_core::with_app;
use std::sync::Arc;
use warp::Filter;

use crate::handler::{
    handle_avatar_pipeline, handle_generate_audio, handle_generate_video, handle_lip_sync,
    handle_remove_background, handle_stretch_video, AvatarPipelineRequest, GenerateAudioRequest,
    GenerateVideoRequest, LipSyncRequest, RemoveBackgroundRequest, StretchVideoRequest,
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
    generate_audio(app.clone())
        .or(generate_video(app.clone()))
        .or(stretch_video(app.clone()))
        .or(lip_sync(app.clone()))
        .or(remove_background(app.clone()))
        .or(pipeline(app))
}

fn generate_audio<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "scenes" / i64 / "avatar" / "generate-audio")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<GenerateAudioRequest>())
        .and_then(handle_generate_audio)
}

fn generate_video<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "scenes" / i64 / "avatar" / "generate-video")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<GenerateVideoRequest>())
        .and_then(handle_generate_video)
}

fn stretch_video<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "scenes" / i64 / "avatar" / "stretch-video")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<StretchVideoRequest>())
        .and_then(handle_stretch_video)
}

fn lip_sync<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "scenes" / i64 / "avatar" / "lip-sync")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<LipSyncRequest>())
        .and_then(handle_lip_sync)
}

fn remove_background<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "scenes" / i64 / "avatar" / "remove-background")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<RemoveBackgroundRequest>())
        .and_then(handle_remove_background)
}

fn pipeline<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "scenes" / i64 / "avatar" / "pipeline")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<AvatarPipelineRequest>())
        .and_then(handle_avatar_pipeline)
}
