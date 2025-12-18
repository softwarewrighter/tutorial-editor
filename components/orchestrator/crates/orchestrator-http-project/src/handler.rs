//! Project request handlers

use orchestrator_app::{
    OrchestratorApp,
    AssetRepository, ProjectRepository, SceneRepository,
};
use orchestrator_ops_project::ProjectOps;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;

/// Request to create a new project
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub slug: String,
    pub title: String,
}

/// Handle list projects request
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

/// Handle create project request
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
