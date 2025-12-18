use crate::filters::with_app;
use crate::handlers::{
    CreateAssetRequest, UpdateAssetRequest, handle_create_asset, handle_delete_asset,
    handle_list_project_assets, handle_list_scene_assets, handle_update_asset,
};
use orchestrator_core::{
    OrchestratorApp,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use std::sync::Arc;
use warp::Filter;

pub fn routes<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    list_by_project(app.clone())
        .or(list_by_scene(app.clone()))
        .or(create(app.clone()))
        .or(update(app.clone()))
        .or(delete(app))
}

fn list_by_project<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "projects" / i64 / "assets")
        .and(warp::get())
        .and(with_app(app))
        .and_then(handle_list_project_assets)
}

fn list_by_scene<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "scenes" / i64 / "assets")
        .and(warp::get())
        .and(with_app(app))
        .and_then(handle_list_scene_assets)
}

fn create<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "assets")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<CreateAssetRequest>())
        .and_then(handle_create_asset)
}

fn update<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "assets" / i64)
        .and(warp::put())
        .and(with_app(app))
        .and(warp::body::json::<UpdateAssetRequest>())
        .and_then(handle_update_asset)
}

fn delete<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "assets" / i64)
        .and(warp::delete())
        .and(with_app(app))
        .and_then(handle_delete_asset)
}
