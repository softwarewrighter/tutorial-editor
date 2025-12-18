//! Route composition

use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use std::sync::Arc;
use warp::Filter;

/// All API routes combined
pub fn all<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    orchestrator_http_health::routes()
        .or(orchestrator_http_project::routes(app.clone()))
        .or(orchestrator_http_scene::routes(app.clone()))
        .or(orchestrator_http_asset::routes(app.clone()))
        .or(orchestrator_http_avatar::routes(app.clone()))
        .or(orchestrator_http_script::routes(app))
}
