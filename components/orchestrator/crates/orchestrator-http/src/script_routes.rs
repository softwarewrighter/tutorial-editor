use crate::routes::with_app;
use orchestrator_core::{
    OrchestratorApp,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use orchestrator_http_handlers::{
    GenerateScriptRequest, handle_generate_project_script, handle_generate_scene_script,
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
    generate_project_script(app.clone()).or(generate_scene_script(app))
}

fn generate_project_script<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "projects" / i64 / "generate-script")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<GenerateScriptRequest>())
        .and_then(handle_generate_project_script)
}

fn generate_scene_script<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    warp::path!("api" / "scenes" / i64 / "generate-script")
        .and(warp::post())
        .and(with_app(app))
        .and(warp::body::json::<GenerateScriptRequest>())
        .and_then(handle_generate_scene_script)
}
