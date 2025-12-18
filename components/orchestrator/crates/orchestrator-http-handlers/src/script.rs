use orchestrator_app::{
    OrchestratorApp,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use orchestrator_script::ScriptOps;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct GenerateScriptRequest {
    pub prompt: String,
}

pub async fn handle_generate_project_script<P, S, A>(
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

pub async fn handle_generate_scene_script<P, S, A>(
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
