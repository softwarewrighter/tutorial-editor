use orchestrator_core::{
    OrchestratorApp,
    ports::{AssetRepository, ProjectRepository, SceneRepository},
};
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub slug: String,
    pub title: String,
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
