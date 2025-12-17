use crate::filters::with_app;
use crate::handlers::{
    CreateSceneRequest, UpdateSceneRequest, handle_create_scene, handle_delete_scene,
    handle_list_scenes, handle_update_scene,
};
use orchestrator_core::{
    OrchestratorApp,
    ports::{ProjectRepository, SceneRepository},
};
use std::sync::Arc;
use warp::Filter;

pub fn routes<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    list(app.clone())
        .or(create(app.clone()))
        .or(update(app.clone()))
        .or(delete(app))
}

fn list<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    warp::path!("api" / "projects" / i64 / "scenes")
        .and(warp::get())
        .and(with_app(app))
        .and_then(handle_list_scenes)
}

fn create<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    warp::path!("api" / "projects" / i64 / "scenes")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<CreateSceneRequest>())
        .and_then(handle_create_scene)
}

fn update<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    warp::path!("api" / "scenes" / i64)
        .and(warp::put())
        .and(with_app(app))
        .and(warp::body::json::<UpdateSceneRequest>())
        .and_then(handle_update_scene)
}

fn delete<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    warp::path!("api" / "scenes" / i64)
        .and(warp::delete())
        .and(with_app(app))
        .and_then(handle_delete_scene)
}
