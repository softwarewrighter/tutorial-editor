//! Asset read request handlers and types

use orchestrator_app::{AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository};
use orchestrator_ops_asset::AssetReadOps;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;

/// Request to create a new asset
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

/// Request to update an existing asset
#[derive(Debug, Deserialize)]
pub struct UpdateAssetRequest {
    pub name: String,
    pub asset_type: String,
    pub file_path: Option<String>,
    pub url: Option<String>,
    pub metadata: Option<String>,
}

/// Handle list project assets request
pub async fn handle_list_project_assets<P, S, A>(
    project_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    Ok(warp::reply::json(&app.list_assets_by_project(project_id).await.unwrap_or_default()))
}

/// Handle list scene assets request
pub async fn handle_list_scene_assets<P, S, A>(
    scene_id: i64,
    app: Arc<OrchestratorApp<P, S, A>>,
) -> Result<impl warp::Reply, Infallible>
where
    P: ProjectRepository + 'static,
    S: SceneRepository + 'static,
    A: AssetRepository + 'static,
{
    Ok(warp::reply::json(&app.list_assets_by_scene(scene_id).await.unwrap_or_default()))
}
