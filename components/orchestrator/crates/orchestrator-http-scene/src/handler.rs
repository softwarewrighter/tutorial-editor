//! Scene request handlers

use orchestrator_app::{
    AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository,
};
use orchestrator_ops_scene::{SceneReadOps, SceneWriteOps};
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;

/// Request to create a new scene
#[derive(Debug, Deserialize)]
pub struct CreateSceneRequest {
    pub title: String,
    pub sort_order: i32,
}

/// Request to update an existing scene
#[derive(Debug, Deserialize)]
pub struct UpdateSceneRequest {
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub script_text: Option<String>,
}

/// Handle list scenes request
pub async fn handle_list_scenes<P, S, A>(
    project_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let scenes = app.list_scenes_by_project(project_id).await.unwrap_or_default();
    Ok(warp::reply::json(&scenes))
}

/// Handle create scene request
pub async fn handle_create_scene<P, S, A>(
    project_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: CreateSceneRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let created = app
        .create_scene(project_id, payload.title, payload.sort_order)
        .await
        .unwrap();
    Ok(warp::reply::json(&created))
}

/// Handle update scene request
pub async fn handle_update_scene<P, S, A>(
    id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: UpdateSceneRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let existing = app.get_scene(id).await.unwrap();
    if let Some(mut scene) = existing {
        scene.title = payload.title;
        scene.description = payload.description;
        scene.sort_order = payload.sort_order;
        scene.script_text = payload.script_text;
        let updated = app.update_scene(scene).await.unwrap();
        Ok(warp::reply::json(&updated))
    } else {
        Ok(warp::reply::json(&serde_json::json!({"error": "not found"})))
    }
}

/// Handle delete scene request
pub async fn handle_delete_scene<P, S, A>(
    id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    app.delete_scene(id).await.unwrap();
    Ok(warp::reply::json(&serde_json::json!({"deleted": true})))
}
