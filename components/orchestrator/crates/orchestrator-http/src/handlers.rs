use orchestrator_core::{
    OrchestratorApp,
    domain::{Asset, Scene},
    ports::{AssetRepository, ProjectRepository, SceneRepository},
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

#[derive(Debug, Deserialize)]
pub struct CreateAssetRequest {
    pub project_id: i64,
    pub scene_id: Option<i64>,
    pub name: String,
    pub asset_type: String,
    pub file_path: Option<String>,
    pub url: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAssetRequest {
    pub name: String,
    pub asset_type: String,
    pub file_path: Option<String>,
    pub url: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateScriptRequest {
    pub prompt: String,
}

pub async fn handle_health() -> Result<impl warp::Reply, Infallible> {
    #[derive(Serialize)]
    struct HealthReply {
        status: &'static str,
    }

    Ok(warp::reply::json(&HealthReply { status: "ok" }))
}

pub async fn handle_list_projects<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let projects = app.list_projects().await.unwrap_or_default();
    Ok(warp::reply::json(&projects))
}

pub async fn handle_create_project<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: CreateProjectRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let created = app
        .create_project(payload.slug, payload.title)
        .await
        .unwrap();
    Ok(warp::reply::json(&created))
}

pub async fn handle_list_scenes<P, S, A>(
    project_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let scenes = app.list_scenes(project_id).await.unwrap_or_default();
    Ok(warp::reply::json(&scenes))
}

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
        Ok(warp::reply::json(&Scene::new(0, String::new(), 0)))
    }
}

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

pub async fn handle_list_project_assets<P, S, A>(
    project_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let assets = app.list_project_assets(project_id).await.unwrap_or_default();
    Ok(warp::reply::json(&assets))
}

pub async fn handle_list_scene_assets<P, S, A>(
    scene_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let assets = app.list_scene_assets(scene_id).await.unwrap_or_default();
    Ok(warp::reply::json(&assets))
}

pub async fn handle_create_asset<P, S, A>(
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: CreateAssetRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let asset = Asset::new(
        payload.project_id,
        payload.scene_id,
        payload.name,
        payload.asset_type,
        payload.file_path,
        payload.url,
        payload.metadata,
    );
    let created = app.create_asset(asset).await.unwrap();
    Ok(warp::reply::json(&created))
}

pub async fn handle_update_asset<P, S, A>(
    id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
    payload: UpdateAssetRequest,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let existing = app.get_asset(id).await.unwrap();
    if let Some(mut asset) = existing {
        asset.name = payload.name;
        asset.asset_type = payload.asset_type;
        asset.file_path = payload.file_path;
        asset.url = payload.url;
        asset.metadata = payload.metadata;
        let updated = app.update_asset(asset).await.unwrap();
        Ok(warp::reply::json(&updated))
    } else {
        Ok(warp::reply::json(&serde_json::json!({"error": "not found"})))
    }
}

pub async fn handle_delete_asset<P, S, A>(
    id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    app.delete_asset(id).await.unwrap();
    Ok(warp::reply::json(&serde_json::json!({"deleted": true})))
}
