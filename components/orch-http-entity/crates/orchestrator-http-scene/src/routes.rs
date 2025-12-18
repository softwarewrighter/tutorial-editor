//! Scene routes

use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_http_core::with_app;
use std::sync::Arc;
use warp::Filter;

use crate::handler::{
    handle_create_scene, handle_delete_scene, handle_list_scenes, handle_update_scene,
    CreateSceneRequest, UpdateSceneRequest,
};

/// All scene routes
pub fn routes<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    read_routes(app.clone()).or(write_routes(app))
}

fn read_routes<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "projects" / i64 / "scenes")
        .and(warp::get())
        .and(with_app(app))
        .and_then(handle_list_scenes)
}

fn write_routes<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let create = warp::path!("api" / "projects" / i64 / "scenes")
        .and(warp::post())
        .and(with_app(app.clone()))
        .and(warp::body::json::<CreateSceneRequest>())
        .and_then(handle_create_scene);
    let update = warp::path!("api" / "scenes" / i64)
        .and(warp::put())
        .and(with_app(app.clone()))
        .and(warp::body::json::<UpdateSceneRequest>())
        .and_then(handle_update_scene);
    let delete = warp::path!("api" / "scenes" / i64)
        .and(warp::delete())
        .and(with_app(app))
        .and_then(handle_delete_scene);
    create.or(update).or(delete)
}
