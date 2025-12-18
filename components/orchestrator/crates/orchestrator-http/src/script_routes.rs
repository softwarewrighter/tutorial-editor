use crate::filters::with_app;
use crate::handlers::GenerateScriptRequest;
use orchestrator_core::{
    OrchestratorApp,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use std::convert::Infallible;
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

async fn handle_generate_project_script<P, S, A>(
    project_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: GenerateScriptRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    match app.generate_project_script(project_id, &payload.prompt).await {
        Ok(scenes) => Ok(warp::reply::json(&scenes)),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"error": e.to_string()}))),
    }
}

async fn handle_generate_scene_script<P, S, A>(
    scene_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: GenerateScriptRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    match app.generate_scene_script(scene_id, &payload.prompt).await {
        Ok(scene) => Ok(warp::reply::json(&scene)),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"error": e.to_string()}))),
    }
}
