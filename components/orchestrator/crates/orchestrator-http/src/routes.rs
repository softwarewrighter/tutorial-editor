use crate::asset_routes;
use crate::avatar_routes;
use crate::project_routes;
use crate::scene_routes;
use crate::script_routes;
use orchestrator_core::{
    OrchestratorApp,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use orchestrator_http_handlers::handle_health;
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

pub fn all<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    health()
        .or(project_routes::routes(app.clone()))
        .or(scene_routes::routes(app.clone()))
        .or(asset_routes::routes(app.clone()))
        .or(script_routes::routes(app.clone()))
        .or(avatar_routes::routes(app))
}

fn health() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "health")
        .and(warp::get())
        .and_then(handle_health)
}

/// Filter to inject the app into handlers
pub fn with_app<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = (Arc<OrchestratorApp<P, S, A>>,), Error = Infallible> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::any().map(move || app.clone())
}
