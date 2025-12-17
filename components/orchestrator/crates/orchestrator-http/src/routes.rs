use crate::handlers::handle_health;
use crate::project_routes;
use crate::scene_routes;
use orchestrator_core::{
    OrchestratorApp,
    ports::{ProjectRepository, SceneRepository},
};
use std::sync::Arc;
use warp::Filter;

pub fn all<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    health()
        .or(project_routes::routes(app.clone()))
        .or(scene_routes::routes(app))
}

fn health() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "health")
        .and(warp::get())
        .and_then(handle_health)
}
