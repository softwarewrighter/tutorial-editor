use orchestrator_app::{
    OrchestratorApp,
    domain::Asset,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use orchestrator_ops_asset::{AssetReadOps, AssetWriteOps};
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;

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

pub async fn handle_list_project_assets<P, S, A>(
    project_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    let assets = app.list_assets_by_project(project_id).await.unwrap_or_default();
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
    let assets = app.list_assets_by_scene(scene_id).await.unwrap_or_default();
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
