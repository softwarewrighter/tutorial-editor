use orchestrator_core::{
    OrchestratorApp,
    domain::Scene,
    ports::{ProjectRepository, SceneRepository},
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSceneRequest {
    pub title: String,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSceneRequest {
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub script_text: Option<String>,
}

pub async fn handle_health() -> Result<impl warp::Reply, Infallible> {
    #[derive(Serialize)]
    struct HealthReply {
        status: &'static str,
    }

    Ok(warp::reply::json(&HealthReply { status: "ok" }))
}

pub async fn handle_list_projects<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    let projects = app.list_projects().await.unwrap_or_default();
    Ok(warp::reply::json(&projects))
}

pub async fn handle_create_project<P, S>(
    app: Arc<OrchestratorApp<P, S>>,
    payload: CreateProjectRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    let created = app
        .create_project(payload.slug, payload.title)
        .await
        .unwrap();
    Ok(warp::reply::json(&created))
}

pub async fn handle_list_scenes<P, S>(
    project_id: i64,
    app: Arc<OrchestratorApp<P, S>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    let scenes = app.list_scenes(project_id).await.unwrap_or_default();
    Ok(warp::reply::json(&scenes))
}

pub async fn handle_create_scene<P, S>(
    project_id: i64,
    app: Arc<OrchestratorApp<P, S>>,
    payload: CreateSceneRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    let created = app
        .create_scene(project_id, payload.title, payload.sort_order)
        .await
        .unwrap();
    Ok(warp::reply::json(&created))
}

pub async fn handle_update_scene<P, S>(
    id: i64,
    app: Arc<OrchestratorApp<P, S>>,
    payload: UpdateSceneRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
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
        Ok(warp::reply::json(&Scene::new(0, String::new(), 0)))
    }
}

pub async fn handle_delete_scene<P, S>(
    id: i64,
    app: Arc<OrchestratorApp<P, S>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
{
    app.delete_scene(id).await.unwrap();
    Ok(warp::reply::json(&serde_json::json!({"deleted": true})))
}
