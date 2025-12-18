//! Asset write request handlers

use crate::{CreateAssetRequest, UpdateAssetRequest};
use orchestrator_app::{
    domain::Asset, AssetRepository, OrchestratorApp, ProjectRepository, SceneRepository,
};
use orchestrator_ops_asset::{AssetReadOps, AssetWriteOps};
use std::convert::Infallible;
use std::sync::Arc;

/// Handle create asset request
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
        payload.project_id, payload.scene_id, payload.name,
        payload.asset_type, payload.file_path, payload.url, payload.metadata,
    );
    Ok(warp::reply::json(&app.create_asset(asset).await.unwrap()))
}

/// Handle update asset request
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
    if let Some(mut a) = app.get_asset(id).await.unwrap() {
        a.name = payload.name;
        a.asset_type = payload.asset_type;
        a.file_path = payload.file_path;
        a.url = payload.url;
        a.metadata = payload.metadata;
        Ok(warp::reply::json(&app.update_asset(a).await.unwrap()))
    } else {
        Ok(warp::reply::json(&serde_json::json!({"error": "not found"})))
    }
}

/// Handle delete asset request
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
