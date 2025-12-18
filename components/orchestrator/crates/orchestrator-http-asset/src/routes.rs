//! Asset routes

use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_http_core::with_app;
use std::sync::Arc;
use warp::Filter;

use crate::handler::{
    handle_list_project_assets, handle_list_scene_assets, CreateAssetRequest, UpdateAssetRequest,
};
use crate::handler_write::{handle_create_asset, handle_delete_asset, handle_update_asset};

/// All asset routes
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
    let list_by_project = warp::path!("api" / "projects" / i64 / "assets")
        .and(warp::get())
        .and(with_app(app.clone()))
        .and_then(handle_list_project_assets);
    let list_by_scene = warp::path!("api" / "scenes" / i64 / "assets")
        .and(warp::get())
        .and(with_app(app))
        .and_then(handle_list_scene_assets);
    list_by_project.or(list_by_scene)
}

fn write_routes<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let create = warp::path!("api" / "assets")
        .and(warp::post())
        .and(with_app(app.clone()))
        .and(warp::body::json::<CreateAssetRequest>())
        .and_then(handle_create_asset);
    let update = warp::path!("api" / "assets" / i64)
        .and(warp::put())
        .and(with_app(app.clone()))
        .and(warp::body::json::<UpdateAssetRequest>())
        .and_then(handle_update_asset);
    let delete = warp::path!("api" / "assets" / i64)
        .and(warp::delete())
        .and(with_app(app))
        .and_then(handle_delete_asset);
    create.or(update).or(delete)
}
