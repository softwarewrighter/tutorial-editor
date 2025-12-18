//! Script request handlers

use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_script::ScriptOps;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;

/// Request to generate a script
#[derive(Debug, Deserialize)]
pub struct GenerateScriptRequest {
    pub prompt: String,
}

/// Handle generate project script request
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

/// Handle generate scene script request
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
